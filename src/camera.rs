use crate::*;
use crate::menu::GameState;
use crate::car::{Car, CameraTarget};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_camera_state)
            .add_systems(Update, camera_follow_system.run_if(in_state(GameState::InGame)));
    }
}

fn setup_camera_state(mut commands: Commands) {
    commands.insert_resource(CameraState::default());
}

#[derive(Resource, Default)]
pub struct CameraState {
    was_reversing: bool,
    stable_timer: f32,
}

fn camera_follow_system(
    car_query: Query<(&Transform, &Car), (With<CameraTarget>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CameraTarget>)>,
    mut camera_state: ResMut<CameraState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((car_transform, car)) = car_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let car_pos = car_transform.translation;
            let car_forward = *car_transform.forward();
            
            // Calculate speed factor (0.0 when idle, 1.0 at max speed)
            let speed_factor = (car.speed.abs() / car.max_speed).clamp(0.0, 1.0);
            
            // Determine if we're actively reversing based on input, not just speed
            let is_actively_reversing = keyboard_input.pressed(KeyCode::ArrowDown) || 
                                      keyboard_input.pressed(KeyCode::KeyS);
            
            // Add stability timer to prevent rapid camera switching
            if camera_state.was_reversing != is_actively_reversing {
                camera_state.stable_timer += time.delta_secs();
                if camera_state.stable_timer > 0.5 { // Only switch after 0.5 seconds
                    camera_state.was_reversing = is_actively_reversing;
                    camera_state.stable_timer = 0.0;
                }
            } else {
                camera_state.stable_timer = 0.0;
            }
            
            // Dynamic camera distance - closer when idle, further when speeding
            let base_distance = 8.0;
            let max_distance = 12.0;
            let camera_distance = base_distance + (max_distance - base_distance) * speed_factor;
            
            // Dynamic camera height
            let base_height = 5.5;
            let min_height = 4.0;
            let camera_height = base_height - (base_height - min_height) * speed_factor;
            
            // Position camera - always try to stay behind the car's movement direction
            let camera_offset = if camera_state.was_reversing {
                // When reversing, position camera in front of the car
                car_forward * camera_distance + Vec3::Y * camera_height
            } else {
                // When moving forward (or idle), position camera behind the car
                -car_forward * camera_distance + Vec3::Y * camera_height
            };
            
            let target_pos = car_pos + camera_offset;
            
            // Smooth camera movement with consistent speed
            let lerp_speed = 0.02; // Consistent, stable movement
            camera_transform.translation = camera_transform.translation.lerp(target_pos, lerp_speed);
            
            // Make camera look at the car with minimal look-ahead
            let look_ahead = if camera_state.was_reversing {
                // When reversing, minimal look-ahead in reverse direction
                -car_forward * speed_factor * 1.5
            } else {
                // When moving forward, minimal look-ahead
                car_forward * speed_factor * 1.5
            };
            
            let look_target = car_pos + Vec3::Y * 1.0 + look_ahead;
            camera_transform.look_at(look_target, Vec3::Y);
        }
    }
} 