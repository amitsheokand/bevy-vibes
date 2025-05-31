use crate::*;
use crate::car::{Car, CameraTarget, Wheel};
use crate::menu::GameState;
use crate::post_processing::RacingPostProcessSettings;
use bevy_rapier3d::prelude::*;
use bevy::gltf::GltfAssetLabel;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_world)
           .add_systems(Update, setup_car_wheels.run_if(in_state(GameState::InGame)))
           .add_systems(OnExit(GameState::InGame), cleanup_world);
    }
}

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct CarModel;

fn cleanup_world(
    mut commands: Commands,
    game_entities: Query<Entity, With<GameEntity>>,
    all_cameras: Query<Entity, With<Camera>>,
) {
    for entity in game_entities.iter() {
        commands.entity(entity).despawn();
    }
    
    // Remove post-processing components from ALL cameras to prevent accumulation
    for camera_entity in all_cameras.iter() {
        commands.entity(camera_entity).remove::<RacingPostProcessSettings>();
        commands.entity(camera_entity).remove::<MotionBlur>();
    }
    
    // Reset ambient light to menu defaults to prevent brightness accumulation
    commands.insert_resource(AmbientLight {
        brightness: 0.3, // Very low for menu
        color: Color::srgb(0.5, 0.5, 0.6), // Neutral menu lighting
        ..default()
    });
    
    // Reset to menu background
    commands.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2)));
}

fn setup_world(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    // Set game background color
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.8, 1.0))); // Blue sky for game
    
    // Add large ground plane
    spawn_ground(&mut commands, &mut meshes, &mut materials);
    
    // Camera is handled by CameraPlugin - don't duplicate here
    
    // Spawn car with GLB model
    let _car_entity = spawn_car(&mut commands, &mut materials, &asset_server);
    
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
        GameEntity, // Mark for cleanup
    ));
}

fn spawn_car(
    commands: &mut Commands,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) -> Entity {
    // Load the GLB car model
    let car_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("cars/sedan-sports.glb"));
    
    // Spawn the car entity with physics and game components
    let car_entity = commands
        .spawn((
            // Start with just the transform and physics - no visual model yet
            Transform::from_xyz(0.0, 0.5, 0.0), // Position so collider bottom touches ground (0.5 - 0.5 = 0.0)
            Visibility::default(), // Add visibility component to prevent warnings
            Car::default(),
            CameraTarget,
            CarModel, // Mark to identify this as the car model for wheel setup
            GameEntity, // Mark for cleanup
        ))
        .insert((
            // Physics components - use appropriate collider for a car
            RigidBody::Dynamic,
            Collider::cuboid(0.7, 0.5, 1.2), // Car collision box - made slightly taller (0.4 -> 0.5)
            AdditionalMassProperties::Mass(500.0), // Reduced mass for better responsiveness
            ExternalForce::default(),
            ExternalImpulse::default(),
            Velocity::default(),
            Friction::coefficient(2.0), // Increased from 1.2 to reduce skidding
            Restitution::coefficient(0.1), // Low bounce
            Damping { linear_damping: 1.0, angular_damping: 2.0 }, // Increased damping to reduce skidding
            LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z, // Prevent car from flipping
        ))
        .with_children(|parent| {
            // Add the GLB model as a child with offset to align with physics collider
            parent.spawn((
                SceneRoot(car_scene),
                Transform::from_xyz(0.0, -0.5, 0.0), // Move model down to align with collider
            ));

            // Add headlights to the car
            let _headlight_material = materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 0.9), // Warm white
                emissive: LinearRgba::new(1.0, 1.0, 0.9, 0.0),
                ..default()
            });

            // Left headlight
            parent.spawn((
                SpotLight {
                    intensity: 5_000_000.0, // Much brighter headlight 
                    color: Color::srgb(1.0, 1.0, 0.9), // Warm white
                    shadows_enabled: true,
                    inner_angle: PI / 8.0, // 22.5 degrees inner cone
                    outer_angle: PI / 4.0, // 45 degrees outer cone
                    range: 400.0, // Much longer range for nighttime driving
                    ..default()
                },
                Transform::from_xyz(-0.5, 0.0, -1.2) // Adjusted Y to account for model offset
                    .looking_at(Vec3::new(-0.5, 0.0, -20.0), Vec3::Y), // Point forward
            ));

            // Right headlight
            parent.spawn((
                SpotLight {
                    intensity: 5_000_000.0, // Much brighter headlight
                    color: Color::srgb(1.0, 1.0, 0.9), // Warm white
                    shadows_enabled: true,
                    inner_angle: PI / 8.0, // 22.5 degrees inner cone
                    outer_angle: PI / 4.0, // 45 degrees outer cone
                    range: 400.0, // Much longer range for nighttime driving
                    ..default()
                },
                Transform::from_xyz(0.5, 0.0, -1.2) // Adjusted Y to account for model offset
                    .looking_at(Vec3::new(0.5, 0.0, -20.0), Vec3::Y), // Point forward
            ));
        })
        .id();

    car_entity
}

// System to find and mark wheel entities by name
fn setup_car_wheels(
    mut commands: Commands,
    car_query: Query<Entity, (With<CarModel>, Without<Wheel>)>,
    children: Query<&Children>,
    names: Query<&Name>,
    wheel_query: Query<Entity, With<Wheel>>,
) {
    // Check if we already have wheels marked
    if wheel_query.iter().count() >= 4 {
        return; // All wheels already marked, no need to continue
    }

    for car_entity in car_query.iter() {
        if let Ok(car_children) = children.get(car_entity) {
            mark_wheels_recursive(&mut commands, car_children, &children, &names, &wheel_query);
        }
    }
}

fn mark_wheels_recursive(
    commands: &mut Commands,
    entities: &Children,
    children: &Query<&Children>,
    names: &Query<&Name>,
    existing_wheels: &Query<Entity, With<Wheel>>,
) {
    for entity in entities.iter() {
        // Skip if this entity is already marked as a wheel
        if existing_wheels.get(entity).is_ok() {
            continue;
        }

        // Check if this entity has a name and if it's a wheel
        if let Ok(name) = names.get(entity) {
            let name_str = name.as_str();
            if name_str == "wheel-back-left" 
                || name_str == "wheel-back-right" 
                || name_str == "wheel-front-left" 
                || name_str == "wheel-front-right" {
                // Mark this entity as a wheel
                commands.entity(entity).insert(Wheel);
                println!("Found and marked wheel: {}", name_str);
            }
        }

        // Recursively check children
        if let Ok(child_entities) = children.get(entity) {
            mark_wheels_recursive(commands, child_entities, children, names, existing_wheels);
        }
    }
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
            GameEntity, // Mark for cleanup
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
            GameEntity, // Mark for cleanup
        ));
    }
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
            GameEntity, // Mark for cleanup
        ));
    }
} 