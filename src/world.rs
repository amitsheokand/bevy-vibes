use crate::*;
use crate::car::{Car, CameraTarget, Wheel};
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Set simple blue sky background
    commands.insert_resource(ClearColor(Color::srgb(0.4, 0.7, 1.0)));
    
    // Add large ground plane
    spawn_ground(&mut commands, &mut meshes, &mut materials);
    
    // Setup camera
    setup_camera(&mut commands);
    
    // Spawn car with wheels and motion blur
    let _car_entity = spawn_car(&mut commands, &mut meshes, &mut materials);
    
    // Create track markers and obstacles
    spawn_track_markers(&mut commands, &mut meshes, &mut materials);
    spawn_obstacles(&mut commands, &mut meshes, &mut materials);
    
    // Add random scattered objects
    spawn_random_objects(&mut commands, &mut meshes, &mut materials);
}

fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // Much larger ground plane for more driving space
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(300.0, 300.0))), // 3x bigger ground
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 0.2), // Green grass color
            perceptual_roughness: 0.9,
            metallic: 0.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0), // At ground level
        // Physics components
        RigidBody::Fixed,
        Collider::cuboid(150.0, 0.1, 150.0), // Large flat collider matching the bigger ground
        Friction::coefficient(0.3), // Reduced friction for easier car movement
    ));
}

fn spawn_car(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    // Make car body narrower so wheels are visible
    let car_body = meshes.add(Mesh::from(Cuboid::new(1.4, 0.8, 3.6))); // Narrower and slightly smaller
    // Make wheels thinner and better proportioned
    let wheel_mesh = meshes.add(Mesh::from(Cylinder::new(0.3, 0.15))); // Thinner wheels
    
    let car_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.1, 0.1), // Red car
        metallic: 0.3,
        perceptual_roughness: 0.7,
        ..default()
    });
    
    let wheel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.1), // Black wheels
        metallic: 0.1,
        perceptual_roughness: 0.9,
        ..default()
    });

    // Spawn the car entity - lower it to sit on ground
    let car_entity = commands
        .spawn((
            Mesh3d(car_body),
            MeshMaterial3d(car_material),
            Transform::from_xyz(0.0, 0.4, 0.0), // Lower to sit on ground (half of car height)
            Car::default(),
            CameraTarget,
            // Physics components
            RigidBody::Dynamic,
            Collider::cuboid(0.7, 0.4, 1.8), // Car collision box
            AdditionalMassProperties::Mass(500.0), // Reduced mass for better responsiveness
            ExternalForce::default(),
            ExternalImpulse::default(),
            Velocity::default(),
            Friction::coefficient(1.2), // Increased friction to reduce skidding
            Restitution::coefficient(0.1), // Low bounce
            Damping { linear_damping: 1.0, angular_damping: 2.0 }, // Increased damping to reduce skidding
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z, // Prevent car from flipping
        ))
        .with_children(|parent| {
            // Position wheels closer to car body and at proper ground level
            let wheel_positions = [
                Vec3::new(-0.75, -0.1, 1.2),  // Front left - proper height for radius 0.3
                Vec3::new(0.75, -0.1, 1.2),   // Front right - proper height for radius 0.3
                Vec3::new(-0.75, -0.1, -1.2), // Rear left - proper height for radius 0.3
                Vec3::new(0.75, -0.1, -1.2),  // Rear right - proper height for radius 0.3
            ];

            for position in wheel_positions.iter() {
                parent.spawn((
                    Mesh3d(wheel_mesh.clone()),
                    MeshMaterial3d(wheel_material.clone()),
                    Transform::from_translation(*position)
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    Wheel, // Add Wheel component for rotation system
                    // Wheel physics - just visual, collision handled by car body
                ));
            }
        })
        .id();

    car_entity
}

fn spawn_track_markers(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
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
            // Physics components
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 1.5, 0.5),
            AdditionalMassProperties::Mass(100.0), // Heavy markers
            Friction::coefficient(0.6),
            Restitution::coefficient(0.2),
        ));
    }
}

fn spawn_obstacles(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // Add more obstacles/buildings spread across the larger area
    for i in 0..8 { // More buildings for bigger area
        let angle = i as f32 * std::f32::consts::PI / 4.0; // 8 buildings instead of 4
        let radius = 40.0 + (i % 3) as f32 * 20.0; // Vary distance: 40, 60, 80 units
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
            // Physics components - heavy buildings
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 3.0, 1.0),
            AdditionalMassProperties::Mass(1000.0), // Very heavy buildings
            Friction::coefficient(0.8),
            Restitution::coefficient(0.1), // Low bounce
        ));
    }
}

fn setup_camera(commands: &mut Commands) {
    // Camera with HDR and motion blur
    commands.spawn((
        Camera3d::default(),
        // HDR for better lighting
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        // Motion blur for realistic speed effects
        MotionBlur {
            shutter_angle: 0.5, // Moderate motion blur
            samples: 4, // Good quality
        },
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

fn spawn_random_objects(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    use std::f32::consts::PI;
    
    // Create different object types
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let sphere_mesh = meshes.add(Sphere::new(0.5));
    let cylinder_mesh = meshes.add(Cylinder::new(0.4, 1.5));
    
    // Different materials
    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        metallic: 0.1,
        perceptual_roughness: 0.7,
        ..default()
    });
    
    let blue_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.8),
        metallic: 0.1,
        perceptual_roughness: 0.7,
        ..default()
    });
    
    let yellow_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.2),
        metallic: 0.1,
        perceptual_roughness: 0.7,
        ..default()
    });
    
    let purple_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.2, 0.8),
        metallic: 0.1,
        perceptual_roughness: 0.7,
        ..default()
    });
    
    // Spawn random objects across the ground
    for i in 0..100 { // More objects for the bigger ground
        // Generate random position avoiding the center track area
        let angle = (i as f32 * 2.3) % (2.0 * PI); // Pseudo-random angle
        let distance = 25.0 + (i as f32 * 1.7) % 75.0; // Spread across much larger distance (25-100 units from center)
        let x = angle.cos() * distance;
        let z = angle.sin() * distance;
        
        // Random object type
        let object_type = i % 4;
        let height = match object_type {
            0 => 0.5,  // Cube half-height
            1 => 0.5,  // Sphere radius
            2 => 0.75, // Cylinder half-height
            _ => 1.0,  // Default
        };
        
        let (mesh, material) = match object_type {
            0 => (cube_mesh.clone(), red_material.clone()),
            1 => (sphere_mesh.clone(), blue_material.clone()),
            2 => (cylinder_mesh.clone(), yellow_material.clone()),
            _ => (cube_mesh.clone(), purple_material.clone()),
        };
        
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(x, height, z)
                .with_rotation(Quat::from_rotation_y((i as f32 * 0.7) % (2.0 * PI))),
            // Physics components for interactive objects
            RigidBody::Dynamic,
            match object_type {
                0 => Collider::cuboid(0.5, 0.5, 0.5), // Cube collider
                1 => Collider::ball(0.5), // Sphere collider
                2 => Collider::cylinder(0.75, 0.4), // Cylinder collider
                _ => Collider::cuboid(0.5, 0.5, 0.5), // Default cube
            },
            AdditionalMassProperties::Mass(match object_type {
                0 => 50.0,  // Cubes - medium weight
                1 => 30.0,  // Spheres - lighter
                2 => 80.0,  // Cylinders - heavier
                _ => 50.0,  // Default
            }),
            Friction::coefficient(0.5),
            Restitution::coefficient(0.3), // Some bounce
        ));
    }
} 