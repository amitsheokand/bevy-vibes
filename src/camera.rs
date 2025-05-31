use crate::*;
use crate::car::{Car, CameraTarget};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_follow_system);
    }
}

fn camera_follow_system(
    car_query: Query<(&Transform, &Car), (With<CameraTarget>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CameraTarget>)>,
) {
    if let Ok((car_transform, car)) = car_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let car_pos = car_transform.translation;
            let car_forward = car_transform.forward();
            
            // Calculate speed factor (0.0 when idle, 1.0 at max speed)
            let speed_factor = (car.speed.abs() / car.max_speed).clamp(0.0, 1.0);
            
            // Dynamic camera distance - closer when idle, further when speeding
            let base_distance = 8.0; // A bit further back when idle
            let max_distance = 12.0; // Further back when at max speed
            let camera_distance = base_distance + (max_distance - base_distance) * speed_factor;
            
            // Dynamic camera height - higher when idle, lower when speeding for action feel
            let base_height = 5.5; // Lower when idle, not on top of car
            let min_height = 4.0; // Lower when speeding for action
            let camera_height = base_height - (base_height - min_height) * speed_factor;
            
            // Position camera behind and above the car with dynamic positioning
            let camera_offset = -car_forward * camera_distance + Vec3::Y * camera_height;
            let target_pos = car_pos + camera_offset;
            
            // Smoothly move camera to target position
            // Faster camera movement when speeding up for responsiveness
            let lerp_speed = 0.03 + speed_factor * 0.02; // 0.03 when idle, 0.05 when at max speed
            camera_transform.translation = camera_transform.translation.lerp(target_pos, lerp_speed);
            
            // Make camera look at the car, slightly ahead when moving fast
            let look_ahead = car_forward * speed_factor * 3.0; // Look ahead when speeding
            let look_target = car_pos + Vec3::Y * 1.0 + look_ahead;
            camera_transform.look_at(look_target, Vec3::Y);
        }
    }
} 