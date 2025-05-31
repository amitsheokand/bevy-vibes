use bevy::prelude::*;
use bevy_vibes::{
    car::CarPlugin,
    camera::CameraPlugin,
    lighting::LightingPlugin,
    world::WorldPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.7, 1.0))) // Nice blue sky color
        .add_plugins((
            CarPlugin,
            CameraPlugin,
            LightingPlugin,
            WorldPlugin,
        ))
        .run();
}
