use bevy::asset::LoadState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

// Import modules
mod components;
mod resources;
mod systems;
mod utils;

// Use components
use components::monster::{MonsterHealthBar, MonsterHealthBarBackground};
use components::{monster::Monster, npc::NPC, player::Player, stats::Stats};
use utils::{calculate_scale, snap_to_grid};

// Use systems
use systems::monster::monster_respawn_system;
use systems::ui::{setup_player_ui, update_monster_health_bars, update_player_ui};
use systems::{combat::combat_system, interaction::npc_interaction, movement::player_movement};

// Import the respawn system
use resources::monster_respawn_timer::MonsterRespawnTimer;

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
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        // Systems that run once when entering the Loading state
        .add_systems(OnEnter(GameState::Loading), load_assets)
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
                update_player_ui,
                update_monster_health_bars,
                monster_respawn_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

// System to load assets during Loading state
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        background: asset_server.load("images/background/background_test.png"),
        player: asset_server.load("images/players/player_test.png"),
        npc: asset_server.load("images/npc/npc_test.png"),
        mask: asset_server.load("images/background/mask_test.png"),
        monster: asset_server.load("images/monsters/monster_test.png"),
    };
    commands.insert_resource(assets);
}

// System to check if assets are loaded and transition to Playing state
fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let background_loaded = asset_server.get_load_state(&assets.background) == LoadState::Loaded;
    let player_loaded = asset_server.get_load_state(&assets.player) == LoadState::Loaded;
    let npc_loaded = asset_server.get_load_state(&assets.npc) == LoadState::Loaded;
    let monster_loaded = asset_server.get_load_state(&assets.monster) == LoadState::Loaded;

    if background_loaded && player_loaded && npc_loaded && monster_loaded {
        next_state.set(GameState::Playing);
    }
}

// System to setup entities once in the Playing state
fn setup(mut commands: Commands, assets: Res<GameAssets>, images: Res<Assets<Image>>) {
    // Add a 2D camera
    commands.spawn(Camera2dBundle::default());

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
    let monster_scale = calculate_scale(&assets.monster, &images);

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
        .insert(Stats::new(100, 10, 5)); // Player stats: health, attack, defense

    // Spawn a monster
    commands
        .spawn(SpriteBundle {
            texture: assets.monster.clone(),
            transform: Transform {
                translation: snap_to_grid(Vec3::new(300.0, 0.0, 0.0)),
                scale: monster_scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Monster)
        .insert(Stats::new(50, 8, 3))
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

    // Insert the monster respawn timer
    commands.insert_resource(MonsterRespawnTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    });

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
