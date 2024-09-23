use bevy::color::palettes::css::{BLUE, DARK_GRAY, RED};
use bevy::prelude::*;

use crate::components::ui::{
    ExperienceBar, ExperienceText, HealthBar, HealthText, LevelText, PlayerUI,
};
use crate::components::{
    monster::Monster, monster::MonsterHealthBar, player::Player, stats::Stats,
};

pub fn setup_player_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a font
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Create the root UI node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column, // Arrange children vertically
                    align_items: AlignItems::FlexStart,    // Align items to the start (left)
                    padding: UiRect::all(Val::Px(10.0)),   // Add some padding around
                    ..Default::default()
                },
                ..Default::default()
            },
            PlayerUI, // Marker component
        ))
        .with_children(|parent| {
            // =====================
            // Health Bar Section
            // =====================
            // Health Bar Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        width: Val::Px(200.0),
                        height: Val::Px(25.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Health Bar Background
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::Srgba(DARK_GRAY)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Health Bar Fill
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0), // Will be updated based on health
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    background_color: BackgroundColor(Color::Srgba(RED)),
                                    ..Default::default()
                                },
                                HealthBar, // Marker component
                            ));
                        });

                    // Health Text Overlay
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "HP: 100 / 100",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 14.0,
                                    color: Color::WHITE,
                                },
                            ),
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(50.0),
                                top: Val::Percent(50.0),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(-50.0, -7.0, 0.0)),
                            ..Default::default()
                        },
                        HealthText, // Marker component
                    ));
                });

            // =====================
            // Experience Bar Section
            // =====================
            // Experience Bar Container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        width: Val::Px(200.0),
                        height: Val::Px(15.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Experience Bar Background
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::Srgba(DARK_GRAY)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Experience Bar Fill
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(0.0), // Will be updated based on experience
                                        height: Val::Percent(100.0),
                                        ..Default::default()
                                    },
                                    background_color: BackgroundColor(Color::Srgba(BLUE)),
                                    ..Default::default()
                                },
                                ExperienceBar, // Marker component
                            ));
                        });

                    // Experience Text Overlay
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "EXP: 0 / 100",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                },
                            ),
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(50.0),
                                top: Val::Percent(50.0),
                                // Adjust the translation to center the text
                                ..Default::default()
                            },
                            // Add `Transform` as a separate component
                            transform: Transform::from_translation(Vec3::new(-50.0, -7.0, 0.0)),
                            ..Default::default()
                        },
                        ExperienceText, // Marker component
                    ));
                });

            // =====================
            // Level Text Section
            // =====================
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Level: 1",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                LevelText, // Marker component
            ));
        });
}

// Separate systems for updating each UI element
pub fn update_health_text(
    player_query: Query<&Stats, With<Player>>,
    mut health_text_query: Query<&mut Text, With<HealthText>>,
) {
    if let Ok(stats) = player_query.get_single() {
        for mut text in health_text_query.iter_mut() {
            text.sections[0].value = format!("HP: {} / {}", stats.health, stats.max_health);
        }
    }
}

pub fn update_experience_text(
    player_query: Query<&Stats, With<Player>>,
    mut experience_text_query: Query<&mut Text, With<ExperienceText>>,
) {
    if let Ok(stats) = player_query.get_single() {
        let exp_next_level = stats.exp_next_level();
        for mut text in experience_text_query.iter_mut() {
            text.sections[0].value = format!("EXP: {} / {}", stats.experience, exp_next_level);
        }
    }
}

pub fn update_level_text(
    player_query: Query<&Stats, With<Player>>,
    mut level_text_query: Query<&mut Text, With<LevelText>>,
) {
    if let Ok(stats) = player_query.get_single() {
        for mut text in level_text_query.iter_mut() {
            text.sections[0].value = format!("Level: {}", stats.level);
        }
    }
}

pub fn update_health_bar(
    player_query: Query<&Stats, With<Player>>,
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
) {
    if let Ok(stats) = player_query.get_single() {
        let health_percentage = (stats.health as f32 / stats.max_health as f32) * 100.0;
        for mut style in health_bar_query.iter_mut() {
            style.width = Val::Percent(health_percentage.clamp(0.0, 100.0));
        }
    }
}

pub fn update_experience_bar(
    player_query: Query<&Stats, With<Player>>,
    mut experience_bar_query: Query<&mut Style, With<ExperienceBar>>,
) {
    if let Ok(stats) = player_query.get_single() {
        let exp_next_level = stats.exp_next_level();
        let exp_percentage = (stats.experience as f32 / exp_next_level as f32) * 100.0;
        for mut style in experience_bar_query.iter_mut() {
            style.width = Val::Percent(exp_percentage.clamp(0.0, 100.0));
        }
    }
}
pub fn update_monster_health_bars(
    monsters: Query<(&Children, &Stats), With<Monster>>,
    mut health_bar_query: Query<&mut Style, With<MonsterHealthBar>>,
) {
    for (children, stats) in monsters.iter() {
        for &child in children.iter() {
            if let Ok(mut style) = health_bar_query.get_mut(child) {
                let health_percentage = (stats.health as f32 / stats.max_health as f32) * 100.0;
                style.width = Val::Percent(health_percentage);
            }
        }
    }
}
