use bevy::color::palettes::css::{RED, WHITE};
use bevy::prelude::*;

pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Dark background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.8), // Semi-transparent black
            custom_size: Some(Vec2::new(2000.0, 2000.0)), // Cover the screen
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 100.0), // Ensure it's on top
        ..Default::default()
    });

    // Game Over text
    commands.spawn(TextBundle {
        text: Text::from_section(
            "Game Over",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"), // Ensure this font exists
                font_size: 100.0,
                color: Color::Srgba(WHITE),
            },
        ),
        style: Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_items: JustifyItems::Center,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 101.0), // Ensure it's above the background
        ..Default::default()
    });
}
