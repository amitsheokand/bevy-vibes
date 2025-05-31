use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_vibes::{
    car::CarPlugin,
    camera::CameraPlugin,
    lighting::LightingPlugin,
    world::WorldPlugin,
    atmosphere::AtmospherePlugin,
    post_processing::PostProcessingPlugin,
    MotionBlur,
};
use bevy_vibes::menu::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2))) // Dark menu background
        .add_plugins((
            MenuPlugin,
            WorldPlugin,
            CarPlugin,
            CameraPlugin,
            LightingPlugin,
            AtmospherePlugin,
            PostProcessingPlugin,
        ))
        .add_systems(OnEnter(GameState::InGame), apply_initial_settings)
        .add_systems(Update, (
            handle_game_input,
            apply_motion_blur_settings,
        ).run_if(in_state(GameState::InGame)))
        .run();
}

fn handle_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    settings: Res<GameSettings>,
    mut camera_query: Query<&mut MotionBlur, With<Camera3d>>,
) {
    // ESC to return to main menu
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
    
    // Toggle motion blur based on settings
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        for mut motion_blur in camera_query.iter_mut() {
            if settings.motion_blur_enabled {
                motion_blur.shutter_angle = 0.5;
                motion_blur.samples = 4;
            } else {
                motion_blur.shutter_angle = 0.0;
                motion_blur.samples = 1;
            }
        }
    }
}

fn apply_initial_settings(
    settings: Res<GameSettings>,
    mut camera_query: Query<&mut MotionBlur, With<Camera3d>>,
) {
    // Apply initial motion blur settings when game starts
    for mut motion_blur in camera_query.iter_mut() {
        if settings.motion_blur_enabled {
            motion_blur.shutter_angle = 0.5;
            motion_blur.samples = 4;
        } else {
            motion_blur.shutter_angle = 0.0;
            motion_blur.samples = 1;
        }
    }
}

fn apply_motion_blur_settings(
    settings: Res<GameSettings>,
    mut camera_query: Query<&mut MotionBlur, With<Camera3d>>,
) {
    // Only run when settings have changed
    if settings.is_changed() {
        for mut motion_blur in camera_query.iter_mut() {
            if settings.motion_blur_enabled {
                motion_blur.shutter_angle = 0.5;
                motion_blur.samples = 4;
            } else {
                motion_blur.shutter_angle = 0.0;
                motion_blur.samples = 1;
            }
        }
    }
}
