use crate::*;

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_lighting);
    }
}

fn setup_lighting(mut commands: Commands) {
    // Configure cascade shadow map for better shadow quality
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 10.0,
        maximum_distance: 50.0,
        ..default()
    }
    .build();

    // Sun (Directional Light) - simplified without atmosphere
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 30000.0, // Bright daylight
            ..default()
        },
        // Sun position affects lighting and shadows
        Transform::from_xyz(10.0, 20.0, 10.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        cascade_shadow_config,
    ));

    // Bright ambient light for good visibility
    commands.insert_resource(AmbientLight {
        brightness: 150.0, // Good ambient light
        color: Color::srgb(0.9, 0.95, 1.0), // Bright daylight tint
        ..default()
    });
} 