use crate::*;
use crate::menu::{GameState, GameSettings};
use crate::car::{Car, CameraTarget};
use crate::world::GameEntity;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_camera_state)
            .add_systems(Update, (camera_follow_system, manage_camera_effects).run_if(in_state(GameState::InGame)));
    }
}

fn setup_camera_state(mut commands: Commands, settings: Res<GameSettings>) {
    commands.insert_resource(CameraState::default());
    
    // Spawn the comprehensive camera with all effects
    commands.spawn((
        Camera3d::default(),
        // HDR is required for atmospheric scattering and better lighting
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Note: Don't add CameraTarget to camera - that's for the car
        // Atmospheric fog for immersive racing experience (balanced)
        DistanceFog {
            color: if settings.atmospheric_fog_enabled {
                Color::srgba(0.3, 0.4, 0.5, 0.6) // Increased intensity but not overwhelming 
            } else {
                Color::srgba(0.3, 0.4, 0.5, 0.0) // Disabled fog
            },
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.3), // Slightly more sun influence  
            directional_light_exponent: 20.0, // Balanced exponent
            falloff: FogFalloff::from_visibility_colors(
                50.0, // Balanced visibility distance (between 25 and 80)
                Color::srgb(0.25, 0.3, 0.4), // More noticeable extinction color
                Color::srgb(0.85, 0.88, 0.92), // Slight blue tint inscattering
            ),
        },
        // Motion blur for realistic speed effects
        MotionBlur {
            shutter_angle: if settings.motion_blur_enabled { 0.5 } else { 0.0 }, // Moderate motion blur
            samples: 4, // Good quality
        },
        // Atmospheric scattering for realistic sky
        Atmosphere::EARTH,
        AtmosphereSettings {
            aerial_view_lut_max_distance: 50000.0, // Scaled for our scene
            scene_units_to_m: 1.0, // Our units are meters
            ..Default::default()
        },
        // Proper exposure for atmospheric scattering
        Exposure::SUNLIGHT,
        // Tone mapping for realistic colors
        Tonemapping::AcesFitted,
        // Bloom for realistic lighting
        Bloom::NATURAL,
        GameEntity,
    ));
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

fn manage_camera_effects(
    settings: Res<GameSettings>,
    mut fog_query: Query<&mut DistanceFog, With<Camera3d>>,
    mut motion_blur_query: Query<&mut MotionBlur, With<Camera3d>>,
) {
    // Update atmospheric fog
    for mut fog in fog_query.iter_mut() {
        // Enable/disable fog with balanced intensity
        if settings.atmospheric_fog_enabled {
            fog.color.set_alpha(0.4); // Balanced fog visibility
        } else {
            fog.color.set_alpha(0.0); // Disable fog
        }
    }
    
    // Update motion blur
    for mut motion_blur in motion_blur_query.iter_mut() {
        if settings.motion_blur_enabled {
            motion_blur.shutter_angle = 0.5; // Enable motion blur
        } else {
            motion_blur.shutter_angle = 0.0; // Disable motion blur
        }
    }
} 