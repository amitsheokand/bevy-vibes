use crate::*;
use crate::menu::GameState;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
    pub motor_force: f32,
    pub brake_force: f32,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            speed: 0.0,
            max_speed: 25.0,
            acceleration: 800.0,
            turn_speed: 3.0,
            motor_force: 15000.0,
            brake_force: 20000.0,
        }
    }
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct Wheel;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (car_physics_system, wheel_rotation_system).run_if(in_state(GameState::InGame)));
    }
}

fn car_physics_system(
    _time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut car_query: Query<(&mut ExternalForce, &ExternalImpulse, &Transform, &mut Car, &Velocity)>,
) {
    for (mut force, _impulse, transform, mut car, velocity) in car_query.iter_mut() {
        // Calculate current speed from velocity
        let current_velocity = velocity.linvel;
        let forward = *transform.forward();
        car.speed = current_velocity.dot(forward);

        // Reset forces each frame
        force.force = Vec3::ZERO;
        force.torque = Vec3::ZERO;

        // Handle forward/backward movement with stronger forces
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            let motor_force = forward * car.motor_force;
            force.force += motor_force;
        } 
        
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            let brake_force = -forward * car.brake_force;
            force.force += brake_force;
        }
        
        // Apply drag/resistance when not accelerating
        if !keyboard_input.pressed(KeyCode::ArrowUp) && !keyboard_input.pressed(KeyCode::KeyW) && 
           !keyboard_input.pressed(KeyCode::ArrowDown) && !keyboard_input.pressed(KeyCode::KeyS) {
            let drag = -current_velocity * 20.0; // Further reduced drag since we increased friction
            force.force += drag;
        }

        // Handle steering with more controlled torque
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            let turn_torque = Vec3::Y * car.turn_speed * 2000.0; // Slightly reduced for better control
            force.torque += turn_torque;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            let turn_torque = Vec3::Y * -car.turn_speed * 2000.0; // Slightly reduced for better control
            force.torque += turn_torque;
        }

        // Apply stronger angular damping for stability
        let angular_damping = -velocity.angvel * 8.0; // Increased for more stability
        force.torque += angular_damping;

        // Limit max speed by applying counter-force
        if current_velocity.length() > car.max_speed {
            let excess_velocity = current_velocity - current_velocity.normalize() * car.max_speed;
            force.force -= excess_velocity * 500.0; // Strong speed limiting force
        }
    }
}

fn wheel_rotation_system(
    time: Res<Time>,
    car_query: Query<&Car>,
    mut wheel_query: Query<&mut Transform, (With<Wheel>, Without<Car>)>,
) {
    if let Ok(car) = car_query.single() {
        let dt = time.delta_secs();
        
        // Calculate wheel rotation based on car speed
        // Wheel circumference affects rotation speed
        let wheel_radius = 0.3;
        let wheel_circumference = 2.0 * PI * wheel_radius;
        let rotation_speed = car.speed / wheel_circumference;
        
        for mut wheel_transform in wheel_query.iter_mut() {
            // Rotate wheels around their local Y axis (proper rolling motion)
            // Negative rotation because forward movement should rotate wheels forward
            wheel_transform.rotate_local_y(-rotation_speed * dt);
        }
    }
} 