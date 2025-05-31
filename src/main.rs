use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            car_movement_system,
            camera_follow_system,
        ))
        .run();
}

#[derive(Component)]
struct Car {
    speed: f32,
    max_speed: f32,
    acceleration: f32,
    turn_speed: f32,
}

#[derive(Component)]
struct CameraTarget;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.7, 0.1))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    
    // car body
    let car_entity = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Car {
            speed: 0.0,
            max_speed: 20.0,
            acceleration: 10.0,
            turn_speed: 2.0,
        },
        CameraTarget,
    )).id();

    // Create wheels for the car
    for (x, z) in [(-0.8, 1.5), (0.8, 1.5), (-0.8, -1.5), (0.8, -1.5)] {
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
            MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
            Transform::from_xyz(x, 0.3, z),
        )).insert(ChildOf(car_entity));
    }

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

    // Simple track markers (cubes) - make them taller so shadows are more visible
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI / 4.0;
        let radius = 15.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 3.0, 1.0))), // Made taller
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.2))),
            Transform::from_xyz(x, 1.5, z), // Raised to match new height
        ));
    }

    // Add some obstacles/buildings around the track for shadow casting
    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI / 2.0;
        let radius = 25.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(2.0, 6.0, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.6, 0.6))),
            Transform::from_xyz(x, 3.0, z),
        ));
    }
    
    // light - reduce ambient lighting to make headlights more prominent
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 500.0, // Reduced intensity
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Reduce ambient light to make headlights more visible
    commands.insert_resource(AmbientLight {
        brightness: 50.0, // Much lower ambient light
        ..default()
    });
}

fn car_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Car)>,
) {
    for (mut transform, mut car) in query.iter_mut() {
        let dt = time.delta_secs();

        // Handle acceleration/deceleration
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            car.speed = (car.speed + car.acceleration * dt).min(car.max_speed);
        } else if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            car.speed = (car.speed - car.acceleration * dt).max(-car.max_speed * 0.5);
        } else {
            // Natural deceleration
            car.speed *= 0.95;
            if car.speed.abs() < 0.1 {
                car.speed = 0.0;
            }
        }

        // Handle steering (only when moving)
        if car.speed.abs() > 0.1 {
            let turn_factor = car.speed / car.max_speed;
            
            if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
                transform.rotate_y(car.turn_speed * turn_factor * dt);
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
                transform.rotate_y(-car.turn_speed * turn_factor * dt);
            }
        }

        // Move the car forward based on its current rotation
        let forward = transform.forward();
        transform.translation += forward * car.speed * dt;
    }
}

fn camera_follow_system(
    car_query: Query<&Transform, (With<CameraTarget>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CameraTarget>)>,
) {
    if let Ok(car_transform) = car_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let car_pos = car_transform.translation;
            let car_forward = car_transform.forward();
            
            // Position camera behind and above the car
            let camera_offset = -car_forward * 8.0 + Vec3::Y * 6.0;
            let target_pos = car_pos + camera_offset;
            
            // Smoothly move camera to target position
            camera_transform.translation = camera_transform.translation.lerp(target_pos, 0.05);
            
            // Make camera look at the car
            let look_target = car_pos + Vec3::Y * 1.0;
            camera_transform.look_at(look_target, Vec3::Y);
        }
    }
}
