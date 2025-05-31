pub mod car;
pub mod camera;
pub mod lighting;
pub mod world;
pub mod atmosphere;
pub mod menu;
pub mod post_processing;

// Re-export commonly used Bevy types
pub use bevy::{
    prelude::*,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping, motion_blur::MotionBlur},
    pbr::{CascadeShadowConfigBuilder, Atmosphere, AtmosphereSettings, light_consts::lux},
    render::camera::Exposure,
};

pub use bevy_rapier3d::prelude::{
    RigidBody, Collider, AdditionalMassProperties, ExternalForce, ExternalImpulse, 
    Velocity, Friction, Restitution, Damping, LockedAxes, RapierPhysicsPlugin,
};
pub use std::f32::consts::PI; 