# Bevy Game Example

A simple 2D game built with Bevy where you control a blue square with arrow keys.

## How to Run

```bash
cargo run
```

## Controls
- **Arrow Keys**: Move the blue square
- **ESC**: Close the game

## What This Demonstrates

- **Bevy Game Engine**: Modern Rust game engine
- **Entity Component System (ECS)**: Bevy's architecture
- **2D Rendering**: Sprite rendering with colored rectangles
- **Input Handling**: Keyboard input processing
- **Game Loop**: Update systems running every frame
- **UI Text**: Displaying instructions on screen

## Features

- Smooth movement with delta time
- Colored sprite rendering (no textures needed)
- Real-time input handling
- Window configuration
- Text rendering

## Game Architecture

```rust
// Component - data attached to entities
#[derive(Component)]
struct Player {
    speed: f32,
}

// System - functions that operate on components
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    // Game logic here
}
```

## Troubleshooting

If the game crashes:
1. **Update graphics drivers**
2. **Try software rendering**: Set `WGPU_BACKEND=gl` environment variable
3. **Check system requirements**: Bevy needs modern graphics support

## Next Steps

- Add more sprites and animations
- Implement collision detection
- Add sound effects
- Create game states (menu, gameplay, game over)
- Add enemies and scoring

The game uses Bevy's Entity Component System (ECS) architecture, which is excellent for game development.