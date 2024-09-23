use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

// Import modules
mod components;
mod resources;
mod systems;
mod utils;

// Use components
use components::monster::{MonsterHealthBar, MonsterHealthBarBackground};
use components::monster_respawn_timer::MonsterRespawnTimer;
use components::stats::{MonsterType, Stats};
use components::timer_component::AttackTimer;
use components::{monster::Monster, npc::NPC, player::Player};

// Use utils
use utils::common::{calculate_scale, calculate_scale_atlas, snap_to_grid};

// Use systems
use crate::systems::ui::{
    setup_player_ui, update_experience_bar, update_experience_text, update_health_bar,
    update_health_text, update_level_text,
};
use systems::animation::animate_sprites;
use systems::loading::check_assets_loaded;
use systems::monster::monster_respawn_system;
use systems::monster_movement::monster_movement_system;
use systems::{
    combat_system::combat_system, interaction::npc_interaction, movement::player_movement,
};

// Use resources
use resources::game_assets::GameAssets;

// Define GameState
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Component)]
struct Mask;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1))) // Set a visible background color
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Slow Life RPG".to_string(), // Set your window title here
                resolution: (1280., 720.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<GameState>()
        // Systems that run once when entering the Loading state
        .add_systems(OnEnter(GameState::Loading), load_game_assets)
        // Systems that run every frame while in the Loading state
        .add_systems(
            Update,
            check_assets_loaded.run_if(in_state(GameState::Loading)),
        )
        // Systems that run once when entering the Playing state
        .add_systems(OnEnter(GameState::Playing), (setup, setup_player_ui))
        // Systems that run every frame while in the Playing state
        .add_systems(
            Update,
            (
                player_movement,
                npc_interaction,
                camera_zoom,
                camera_follow_player,
                mask_follow_player,
                combat_system,
                update_experience_bar,
                update_experience_text,
                update_health_bar,
                update_health_text,
                update_level_text,
                monster_respawn_system,
                animate_sprites,
                monster_movement_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

// System to setup entities once in the Playing state
fn setup(
    mut commands: Commands,
    assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    // println!("Entering setup function.");
    // Spawn the camera with an initial zoom of 0.4
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(0.6, 0.6, 1.0), // Set zoom to 0.8as
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn the background
    commands.spawn(SpriteBundle {
        texture: assets.background.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0), // Behind the player
            ..Default::default()
        },
        ..Default::default()
    });

    // Calculate scales
    let player_scale = calculate_scale(&assets.player, &images);
    let npc_scale = calculate_scale(&assets.npc, &images);
    let monster_scale = calculate_scale_atlas(&assets.monster_sprite_sheet, &texture_atlases);

    // Spawn the player
    commands
        .spawn(SpriteBundle {
            texture: assets.player.clone(),
            transform: Transform {
                translation: snap_to_grid(Vec3::new(0.0, 0.0, 0.0)),
                scale: player_scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Stats::player_stats(100, 10, 5));

    // Spawn a monster
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: assets.monster_sprite_sheet.clone(),
            transform: Transform {
                translation: snap_to_grid(Vec3::new(300.0, 0.0, 0.0)),
                scale: monster_scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Monster)
        .insert(Stats::monster_stats(MonsterType::Lesser))
        .insert(AttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .with_children(|parent| {
            // Health Bar Background
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(32.0),
                            height: Val::Px(5.0),
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(40.0), // Position above the monster
                            // Optionally set left or right
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::DARK_GRAY),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..Default::default()
                    },
                    MonsterHealthBarBackground,
                ))
                .with_children(|parent| {
                    // Health Bar Fill
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::GREEN),
                            ..Default::default()
                        },
                        MonsterHealthBar,
                    ));
                });
        });

    // To manage the respawn timing of monsters globally.
    commands.insert_resource(MonsterRespawnTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));

    // Spawn an NPC
    commands
        .spawn(SpriteBundle {
            texture: assets.npc.clone(),
            transform: Transform {
                translation: snap_to_grid(Vec3::new(200.0, 0.0, 0.0)),
                scale: npc_scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NPC);

    // Spawn the mask sprite
    commands
        .spawn(SpriteBundle {
            texture: assets.mask.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0), // Set a high Z value to render on top
                scale: Vec3::ONE,                       // Adjust scale if necessary
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::WHITE, // Ensure full color is rendered
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Mask);
}

fn camera_zoom(
    mut query: Query<&mut OrthographicProjection, With<Camera2d>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let zoom_speed = 0.1; // Adjust zoom speed as needed

    for mut ortho in query.iter_mut() {
        // Zoom using the scroll wheel
        for event in scroll_evr.iter() {
            ortho.scale -= event.y * zoom_speed;
            ortho.scale = ortho.scale.clamp(0.4, 0.6); // Limit the zoom levels
        }

        // Zoom using keys (e.g., Z to zoom in, X to zoom out)
        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale -= zoom_speed;
            ortho.scale = ortho.scale.clamp(0.4, 0.6);
        }
        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale += zoom_speed;
            ortho.scale = ortho.scale.clamp(0.4, 0.6);
        }
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for mut camera_transform in camera_query.iter_mut() {
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

fn mask_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Mask>)>, // Added Without<Mask>
    mut mask_query: Query<&mut Transform, With<Mask>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut mask_transform in mask_query.iter_mut() {
            mask_transform.translation.x = player_transform.translation.x;
            mask_transform.translation.y = player_transform.translation.y;
        }
    }
}

/// System to load GameAssets during Startup
fn load_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // println!("Loading game assets...");
    let game_assets = GameAssets::new(&asset_server, &mut texture_atlases);
    commands.insert_resource(game_assets);
    // println!("Game assets resource inserted.");
}
