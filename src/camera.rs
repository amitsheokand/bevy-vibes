use crate::*;
use crate::car::{Car, CameraTarget};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraState::default())
            .add_systems(Update, camera_follow_system);
    }
}

#[derive(Resource, Default)]
pub struct CameraState {
    was_reversing: bool,
}

fn camera_follow_system(
    car_query: Query<(&Transform, &Car), (With<CameraTarget>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CameraTarget>)>,
    mut camera_state: ResMut<CameraState>,
) {
    if let Ok((car_transform, car)) = car_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let car_pos = car_transform.translation;
            let car_forward = car_transform.forward();
            
            // Calculate speed factor (0.0 when idle, 1.0 at max speed)
            let speed_factor = (car.speed.abs() / car.max_speed).clamp(0.0, 1.0);
            
            // Determine if we're moving backwards
            let is_reversing = car.speed < -0.1;
            
            // Check if direction changed for instant switching
            let direction_changed = camera_state.was_reversing != is_reversing;
            camera_state.was_reversing = is_reversing;
            
            // Dynamic camera distance - closer when idle, further when speeding
            let base_distance = 8.0; // A bit further back when idle
            let max_distance = 12.0; // Further back when at max speed
            let camera_distance = base_distance + (max_distance - base_distance) * speed_factor;
            
            // Dynamic camera height - higher when idle, lower when speeding for action feel
            let base_height = 5.5; // Lower when idle, not on top of car
            let min_height = 4.0; // Lower when speeding for action
            let camera_height = base_height - (base_height - min_height) * speed_factor;
            
            // Position camera based on direction
            let camera_offset = if is_reversing {
                // When reversing, position camera in front of the car
                car_forward * camera_distance + Vec3::Y * camera_height
            } else {
                // When moving forward, position camera behind the car
                -car_forward * camera_distance + Vec3::Y * camera_height
            };
            
            let target_pos = car_pos + camera_offset;
            
            // Move camera - instant switch on direction change, smooth otherwise
            if direction_changed {
                // Instant switch when changing direction
                camera_transform.translation = target_pos;
            } else {
                // Smoothly move camera to target position during normal movement
                let lerp_speed = 0.03 + speed_factor * 0.02; // 0.03 when idle, 0.05 when at max speed
                camera_transform.translation = camera_transform.translation.lerp(target_pos, lerp_speed);
            }
            
            // Make camera look at the car, with appropriate look-ahead
            let look_ahead = if is_reversing {
                // When reversing, look slightly behind the car (in the direction of movement)
                -car_forward * speed_factor * 3.0
            } else {
                // When moving forward, look slightly ahead
                car_forward * speed_factor * 3.0
            };
            
            let look_target = car_pos + Vec3::Y * 1.0 + look_ahead;
            camera_transform.look_at(look_target, Vec3::Y);
        }
    }
} 