use crate::*;

pub struct AtmospherePlugin;

impl Plugin for AtmospherePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeOfDay::default())
            .add_systems(Startup, setup_atmosphere)
            .add_systems(Update, (update_time_of_day, update_sun_position));
    }
}

#[derive(Resource)]
pub struct TimeOfDay {
    pub time: f32, // 0.0 = midnight, 0.5 = noon, 1.0 = midnight again
    pub speed: f32, // How fast time passes
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            time: 0.3, // Start at morning
            speed: 1.0 / (24.0 * 60.0), // 24 minutes = 1 full day cycle (≈0.0007)
        }
    }
}

#[derive(Component)]
pub struct Sun;

fn setup_atmosphere(_commands: Commands) {
    // No need to setup sun here, it's done in lighting module
    // This system will control the existing sun
}

fn update_time_of_day(
    mut time_of_day: ResMut<TimeOfDay>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // Allow manual time control
    if keyboard_input.pressed(KeyCode::KeyT) {
        time_of_day.time += time.delta_secs() * 2.0; // Fast forward
    } else if keyboard_input.pressed(KeyCode::KeyG) {
        time_of_day.time -= time.delta_secs() * 2.0; // Fast backward
    } else {
        // Automatic time progression
        time_of_day.time += time.delta_secs() * time_of_day.speed;
    }
    
    // Wrap time between 0.0 and 1.0
    time_of_day.time = time_of_day.time.fract();
    if time_of_day.time < 0.0 {
        time_of_day.time += 1.0;
    }
}

fn update_sun_position(
    mut sun_query: Query<&mut Transform, With<DirectionalLight>>,
    mut light_query: Query<&mut DirectionalLight>,
    time_of_day: Res<TimeOfDay>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    let time = time_of_day.time;
    
    // Calculate sun position (arc across sky)
    let sun_angle = (time - 0.25) * 2.0 * PI; // 0.25 offset so noon is high
    let sun_height = sun_angle.sin().max(0.0); // Don't go below horizon
    let sun_forward = sun_angle.cos();
    
    // Update sun transform
    if let Ok(mut sun_transform) = sun_query.single_mut() {
        let sun_direction = Vec3::new(sun_forward, sun_height, 0.3);
        *sun_transform = Transform::from_translation(sun_direction * 100.0)
            .looking_at(Vec3::ZERO, Vec3::Y);
    }
    
    // Update sun intensity based on time of day
    if let Ok(mut directional_light) = light_query.single_mut() {
        let base_intensity = if sun_height > 0.1 {
            // Daylight
            lux::RAW_SUNLIGHT * sun_height.powf(0.5)
        } else if sun_height > 0.0 {
            // Sunset/sunrise
            lux::RAW_SUNLIGHT * 0.1 * (sun_height / 0.1).powf(2.0)
        } else {
            // Night
            100.0 // Minimal moonlight
        };
        
        directional_light.illuminance = base_intensity;
        
        // Change sun color based on height
        directional_light.color = if sun_height > 0.5 {
            Color::WHITE // High noon - white light
        } else if sun_height > 0.1 {
            Color::srgb(1.0, 0.9, 0.7) // Lower sun - warmer light
        } else if sun_height > 0.0 {
            Color::srgb(1.0, 0.5, 0.3) // Sunset/sunrise - orange
        } else {
            Color::srgb(0.3, 0.3, 0.7) // Night - blue moonlight
        };
    }
    
    // Update ambient light
    let ambient_intensity = if sun_height > 0.0 {
        300.0 * sun_height.powf(0.3) // Bright ambient during day
    } else {
        150.0 // Brighter blue ambient at night for better visibility
    };
    
    ambient_light.brightness = ambient_intensity;
    ambient_light.color = if sun_height > 0.1 {
        Color::srgb(0.9, 0.95, 1.0) // Daylight ambient
    } else {
        Color::srgb(0.2, 0.3, 0.8) // Stronger blue night ambient (moonlight effect)
    };
} 