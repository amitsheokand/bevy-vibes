 # Bevy Car Racing Game 🏎️

A 3D car racing game built with Bevy Engine, featuring dynamic lighting, realistic physics, and modular architecture.

## Features

- **Dynamic Car Controls**: WASD/Arrow keys for movement with realistic physics
- **Adaptive Camera**: Distance and height adjust based on speed for cinematic experience
- **Headlight System**: Functioning spotlights with shadow casting
- **Beautiful Environment**: Blue sky, realistic lighting with HDR and bloom
- **Modular Architecture**: Clean plugin-based code organization

## Controls

- **W / ↑**: Accelerate forward
- **S / ↓**: Reverse/brake
- **A / ←**: Turn left (when moving)
- **D / →**: Turn right (when moving)

## Project Structure

The game follows Bevy and Rust best practices with a modular plugin architecture:

### Core Files

- **`src/main.rs`**: Entry point and app setup
- **`src/lib.rs`**: Module declarations and common imports

### Game Modules

- **`src/car.rs`**: Car component and movement system
  - `Car` component with speed, acceleration, turn speed
  - `CameraTarget` marker component
  - `CarPlugin` with movement system

- **`src/camera.rs`**: Dynamic camera follow system
  - Speed-based camera positioning
  - Smooth transitions and look-ahead
  - `CameraPlugin` with follow system

- **`src/lighting.rs`**: Lighting setup and management
  - Sun/directional light with shadows
  - Ambient lighting configuration
  - Headlight creation utility
  - `LightingPlugin` for lighting setup

- **`src/world.rs`**: Scene and world generation
  - Car spawning with wheels
  - Track markers and obstacles
  - Ground plane and camera setup
  - `WorldPlugin` for world initialization

## Technical Features

### Camera System
- **Idle**: 8 units back, 5.5 units high
- **Speeding**: 12 units back, 4 units high  
- **Look-ahead**: Camera looks forward when moving fast
- **Smooth interpolation**: Based on speed factor

### Lighting
- **HDR Pipeline**: High Dynamic Range for realistic lighting
- **Bloom Effects**: Natural light bleeding
- **Tone Mapping**: ACES for cinematic colors
- **Dynamic Shadows**: Both sun and headlight shadows
- **Cascade Shadow Maps**: Optimized shadow quality

### Architecture
- **Plugin System**: Each module has its own plugin
- **Component-Based**: ECS architecture with clear separation
- **Resource Management**: Proper asset and resource handling
- **Modular Design**: Easy to extend and maintain

## Building and Running

```bash
# Run debug game
cargo run --release

# run release build
cargo run --release
```

## Requirements

- Rust 1.70+
- Bevy 0.16.1
- GPU with modern graphics support

## Code Organization Benefits

1. **Maintainability**: Each system in its own module
2. **Reusability**: Plugins can be easily reused
3. **Testing**: Systems can be tested independently  
4. **Collaboration**: Multiple developers can work on different modules
5. **Extension**: Easy to add new features as plugins

## Future Enhancements

- Track system with checkpoints
- Multiple car models
- Sound effects and music
- Particle effects for exhaust/dust
- Multiplayer support
- AI opponents