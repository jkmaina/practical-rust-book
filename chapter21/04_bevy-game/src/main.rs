use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game Example - Use Arrow Keys to Move".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera
    commands.spawn(Camera2dBundle::default());
    
    // Try to load player sprite, fallback to colored rectangle
    let player_texture = asset_server.load("player.png");
    
    // Add the player sprite
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.4, 1.0), // Blue color as fallback
                custom_size: Some(Vec2::new(64.0, 64.0)),
                ..default()
            },
            texture: player_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player { speed: 200.0 },
    ));
    
    // Add instructions text
    commands.spawn(
        TextBundle::from_section(
            "Use Arrow Keys to Move the Blue Square",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * player.speed * time.delta_seconds();
        }
    }
}