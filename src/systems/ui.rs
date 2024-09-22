use crate::components::{
    monster::Monster, monster::MonsterHealthBar, player::Player, stats::Stats,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerUI;

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct ExperienceBar;

#[derive(Component)]
pub struct LevelText;

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
                    flex_direction: FlexDirection::Column, // Set to Column
                    align_items: AlignItems::FlexStart,    // Align items to the start
                    ..Default::default()
                },
                ..Default::default()
            },
            PlayerUI,
        ))
        .with_children(|parent| {
            // Health Bar Background
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Health Bar Fill
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::RED),
                            ..Default::default()
                        },
                        HealthBar,
                    ));
                });

            // Experience Bar Background
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(10.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Experience Bar Fill
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::BLUE),
                            ..Default::default()
                        },
                        ExperienceBar,
                    ));
                });

            // Level Text
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Level: 1",
                        TextStyle {
                            font,
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
                LevelText,
            ));
        });
}

pub fn update_player_ui(
    player_query: Query<&Stats, With<Player>>,
    mut health_bar_query: Query<&mut Style, (With<HealthBar>, Without<ExperienceBar>)>,
    mut experience_bar_query: Query<&mut Style, (With<ExperienceBar>, Without<HealthBar>)>,
    mut level_text_query: Query<&mut Text, With<LevelText>>,
) {
    if let Ok(player_stats) = player_query.get_single() {
        // Update Health Bar
        if let Ok(mut style) = health_bar_query.get_single_mut() {
            let health_percentage =
                (player_stats.health as f32 / player_stats.max_health as f32) * 100.0;
            style.width = Val::Percent(health_percentage);
        }

        // Update Experience Bar
        if let Ok(mut style) = experience_bar_query.get_single_mut() {
            let exp_needed = player_stats.level * 100;
            let exp_percentage = (player_stats.experience as f32 / exp_needed as f32) * 100.0;
            style.width = Val::Percent(exp_percentage);
        }

        // Update Level Text
        if let Ok(mut text) = level_text_query.get_single_mut() {
            text.sections[0].value = format!("Level: {}", player_stats.level);
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
