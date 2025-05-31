use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_vibes::{
    car::CarPlugin,
    camera::CameraPlugin,
    lighting::LightingPlugin,
    world::WorldPlugin,
    MotionBlur,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 1.0))) // Nice blue sky color
        .add_plugins((
            WorldPlugin,
            CarPlugin,
            CameraPlugin,
            LightingPlugin,
        ))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, ui_system)
        .run();
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Motion Blur Racing Game\nWASD/Arrow Keys to drive\n[1] Toggle Motion Blur\n[ESC] Exit"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

fn ui_system(
    mut motion_blur_query: Query<&mut MotionBlur>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.write(AppExit::Success);
    }
    
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        if let Ok(mut motion_blur) = motion_blur_query.single_mut() {
            // Toggle motion blur
            motion_blur.samples = if motion_blur.samples > 0 { 0 } else { 4 };
        }
    }
}
