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
            max_speed: 60.0, // ~240 km/h (realistic M-series top speed)
            acceleration: 800.0, // Responsive but not instant
            turn_speed: 2.5, // Balanced steering response
            motor_force: 35000.0, // Much stronger force for heavier car (1700kg)
            brake_force: 25000.0, // Strong braking for heavy car
        }
    }
}

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct Wheel;

#[derive(Component)]
pub struct FrontWheel; // Component to mark front wheels for steering

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (car_physics_system, wheel_rotation_system, front_wheel_steering_system).run_if(in_state(GameState::InGame)));
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
        let right = *transform.right();
        car.speed = current_velocity.dot(forward);

        // Reset forces each frame
        force.force = Vec3::ZERO;
        force.torque = Vec3::ZERO;

        // Calculate current speed percentage for acceleration curve
        let speed_percentage = (current_velocity.length() / car.max_speed).clamp(0.0, 1.0);
        
        // BMW M-series style acceleration curve - strong initial punch, then levels off
        let acceleration_curve = if speed_percentage < 0.3 {
            1.0 // Full power in low speeds (0-30% of max speed)
        } else if speed_percentage < 0.6 {
            0.8 // Good power in mid speeds (30-60%)
        } else if speed_percentage < 0.85 {
            0.5 // Reduced power at high speeds (60-85%)
        } else {
            0.2 // Minimal power near top speed (85-100%)
        };

        // Handle forward/backward movement with realistic acceleration curve
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            let motor_force = forward * car.motor_force * acceleration_curve;
            force.force += motor_force;
        } 
        
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            let brake_force = -forward * car.brake_force;
            force.force += brake_force;
        }
        
        // Lateral grip - prevent sliding sideways (key for reducing skidding feeling)
        let lateral_velocity = current_velocity.dot(right);
        if lateral_velocity.abs() > 0.1 {
            let lateral_grip = -right * lateral_velocity * 15.0; // Strong lateral grip
            force.force += lateral_grip;
        }
        
        // Simple drag when coasting
        if !keyboard_input.pressed(KeyCode::ArrowUp) && !keyboard_input.pressed(KeyCode::KeyW) && 
           !keyboard_input.pressed(KeyCode::ArrowDown) && !keyboard_input.pressed(KeyCode::KeyS) {
            let drag = -current_velocity * 2.0; // Reduced drag for better momentum
            force.force += drag;
        }

        // Improved turning - more responsive and proper for arcade racing
        let turn_effectiveness = (1.0 - speed_percentage * 0.2).max(0.6); // Less reduction at high speed
        let base_turn_force = car.turn_speed * 4000.0; // Stronger turning force
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            let turn_torque = Vec3::Y * base_turn_force * turn_effectiveness;
            force.torque += turn_torque;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            let turn_torque = Vec3::Y * -base_turn_force * turn_effectiveness;
            force.torque += turn_torque;
        }

        // Balanced stability - prevent spinning but allow responsive turning
        let angular_damping = -velocity.angvel * 5.0; // Reduced for better turning response
        force.torque += angular_damping;

        // Add slight downforce at speed to keep car planted
        if speed_percentage > 0.2 {
            let downforce = Vec3::Y * -speed_percentage * 1000.0;
            force.force += downforce;
        }

        // Simple speed limiting
        if current_velocity.length() > car.max_speed {
            let excess_velocity = current_velocity - current_velocity.normalize() * car.max_speed;
            force.force -= excess_velocity * 800.0;
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
        // BMW M-series typically has 18-19" wheels
        let wheel_radius = 0.35; // Realistic wheel radius for M-series
        let wheel_circumference = 2.0 * PI * wheel_radius;
        let rotation_speed = car.speed / wheel_circumference;
        
        for mut wheel_transform in wheel_query.iter_mut() {
            // Rotate wheels around their local X axis (proper rolling motion for car wheels)
            // Negative rotation because forward movement should rotate wheels forward
            wheel_transform.rotate_local_x(-rotation_speed * dt);
        }
    }
}

fn front_wheel_steering_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut front_wheel_query: Query<&mut Transform, (With<FrontWheel>, Without<Car>)>,
) {
    // Calculate steering angle based on input
    let max_steering_angle = 30.0_f32.to_radians(); // 30 degrees max steering
    let mut target_steering = 0.0;
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        target_steering = max_steering_angle; // Turn left
    } else if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        target_steering = -max_steering_angle; // Turn right
    }
    
    for mut front_wheel_transform in front_wheel_query.iter_mut() {
        // Reset rotation and apply both rolling and steering
        // For front wheels, we need to apply steering rotation around Y-axis
        // The rolling rotation (X-axis) is handled by the main wheel_rotation_system
        
        // Extract current rotation to preserve rolling motion
        let current_rotation = front_wheel_transform.rotation;
        
        // Apply steering by setting the Y rotation directly while preserving X rotation (rolling)
        let euler = current_rotation.to_euler(EulerRot::XYZ);
        front_wheel_transform.rotation = Quat::from_euler(EulerRot::XYZ, euler.0, target_steering, euler.2);
    }
} 