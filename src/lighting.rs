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

pub fn create_headlights(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    car_entity: Entity,
) {
    // Create headlights (spotlights)
    let headlight_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 0.9), // Warm white
        emissive: LinearRgba::new(1.0, 1.0, 0.9, 0.0),
        ..default()
    });

    // Left headlight
    commands.spawn((
        SpotLight {
            intensity: 100_000.0, // lumens
            color: Color::srgb(1.0, 1.0, 0.9), // Warm white
            shadows_enabled: true,
            inner_angle: PI / 6.0, // 30 degrees inner cone
            outer_angle: PI / 4.0, // 45 degrees outer cone
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(-0.6, 0.4, -1.8) // Left front of car (negative Z is forward)
            .looking_at(Vec3::new(-0.6, 0.0, -10.0), Vec3::Y), // Point forward
    )).insert(ChildOf(car_entity))
    .with_children(|builder| {
        // Visible headlight bulb
        builder.spawn((
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(16, 8))),
            MeshMaterial3d(headlight_material.clone()),
        ));
    });

    // Right headlight
    commands.spawn((
        SpotLight {
            intensity: 100_000.0, // lumens
            color: Color::srgb(1.0, 1.0, 0.9), // Warm white
            shadows_enabled: true,
            inner_angle: PI / 6.0, // 30 degrees inner cone
            outer_angle: PI / 4.0, // 45 degrees outer cone
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(0.6, 0.4, -1.8) // Right front of car (negative Z is forward)
            .looking_at(Vec3::new(0.6, 0.0, -10.0), Vec3::Y), // Point forward
    )).insert(ChildOf(car_entity))
    .with_children(|builder| {
        // Visible headlight bulb
        builder.spawn((
            Mesh3d(meshes.add(Sphere::new(0.1).mesh().uv(16, 8))),
            MeshMaterial3d(headlight_material.clone()),
        ));
    });
} 