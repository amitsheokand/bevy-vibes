use crate::*;

#[derive(Component)]
pub struct Car {
    pub speed: f32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            speed: 0.0,
            max_speed: 20.0,
            acceleration: 10.0,
            turn_speed: 2.0,
        }
    }
}

#[derive(Component)]
pub struct CameraTarget;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, car_movement_system);
    }
}

fn car_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Car)>,
) {
    for (mut transform, mut car) in query.iter_mut() {
        let dt = time.delta_secs();

        // Handle acceleration/deceleration
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            car.speed = (car.speed + car.acceleration * dt).min(car.max_speed);
        } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            car.speed = (car.speed - car.acceleration * dt).max(-car.max_speed * 0.5);
        } else {
            // Natural deceleration
            car.speed *= 0.95;
            if car.speed.abs() < 0.1 {
                car.speed = 0.0;
            }
        }

        // Handle steering (only when moving)
        if car.speed.abs() > 0.1 {
            let turn_factor = car.speed / car.max_speed;
            
            if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
                transform.rotate_y(car.turn_speed * turn_factor * dt);
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
                transform.rotate_y(-car.turn_speed * turn_factor * dt);
            }
        }

        // Move the car forward based on its current rotation
        let forward = transform.forward();
        transform.translation += forward * car.speed * dt;
    }
} 