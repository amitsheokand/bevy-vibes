use crate::*;
use crate::menu::GameState;
use crate::world::GameEntity;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_lighting);
    }
}

#[derive(Component)]
pub struct GameLight;

fn setup_lighting(mut commands: Commands) {
    // Configure cascade shadow map for better shadow quality
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 10.0,
        maximum_distance: 50.0,
        ..default()
    }
    .build();

    // Sun (Directional Light) - using raw sunlight for atmospheric scattering
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT, // Raw sunlight for atmospheric scattering
            color: Color::WHITE,
            ..default()
        },
        // Initial sun position (will be controlled by atmosphere system)
        Transform::from_xyz(10.0, 20.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config,
        GameLight, // Mark for cleanup
        GameEntity, // Mark for cleanup
    ));

    // Ambient light (will be controlled by atmosphere system)
    commands.insert_resource(AmbientLight {
        brightness: 300.0, // Starting brightness
        color: Color::srgb(0.9, 0.95, 1.0), // Daylight tint
        ..default()
    });
} 