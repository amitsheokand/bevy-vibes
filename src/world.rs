use crate::*;
use crate::car::{Car, CameraTarget};
use crate::lighting::create_headlights;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.7, 0.1),
            ..default()
        })),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    
    // Car body
    let car_entity = spawn_car(&mut commands, &mut meshes, &mut materials);
    
    // Add headlights to the car
    create_headlights(&mut commands, &mut meshes, &mut materials, car_entity);
    
    // Create track markers and obstacles
    spawn_track_markers(&mut commands, &mut meshes, &mut materials);
    spawn_obstacles(&mut commands, &mut meshes, &mut materials);
    
    // Setup camera
    setup_camera(&mut commands);
}

fn spawn_car(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let car_entity = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            metallic: 0.1,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Car::default(),
        CameraTarget,
    )).id();

    // Create wheels for the car
    for (x, z) in [(-0.8, 1.5), (0.8, 1.5), (-0.8, -1.5), (0.8, -1.5)] {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.1),
                metallic: 0.0,
                perceptual_roughness: 0.9,
                ..default()
            })),
            Transform::from_xyz(x, 0.3, z),
        )).insert(ChildOf(car_entity));
    }

    car_entity
}

fn spawn_track_markers(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Simple track markers (cubes) - make them taller so shadows are more visible
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        let radius = 15.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))), // Made taller
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.8, 0.2),
                metallic: 0.0,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(x, 1.5, z), // Raised to match new height
        ));
    }
}

fn spawn_obstacles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Add some obstacles/buildings around the track for shadow casting
    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI / 2.0;
        let radius = 25.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 6.0, 2.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.6, 0.6, 0.6),
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            })),
            Transform::from_xyz(x, 3.0, z),
        ));
    }
}

fn setup_camera(commands: &mut Commands) {
    // Camera with HDR but simplified (no atmosphere)
    commands.spawn((
        Camera3d::default(),
        // HDR for better lighting
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Proper exposure for daylight
        Exposure {
            ev100: 13.0, // Bright daylight
        },
        // Tone mapping for realistic colors
        Tonemapping::AcesFitted,
        // Bloom for realistic lighting
        Bloom::NATURAL,
    ));
} 