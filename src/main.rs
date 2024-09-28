use bevy::color::palettes::css::DARK_GRAY;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::prelude::*;

// Import modules
mod components;
mod resources;
mod systems;
mod utils;

// Use components
use crate::components::animation::{AnimationIndices, AnimationTimer};
use crate::components::monster_type::MonsterType;
use components::monster::{MonsterHealthBar, MonsterHealthBarBackground};
use components::monster_movement::MonsterMovement;
use components::monster_respawn_timer::MonsterRespawnTimer;
use components::monster_state::MonsterState;
use components::stats::Stats;
use components::timer_component::{AttackTimer, MovementTimer};
use components::{monster::Monster, npc::NPC, player::Player};

// Use utils
use utils::common::{calculate_scale, snap_to_grid};

// Use systems
use crate::systems::ui::{
    setup_player_ui, update_experience_bar, update_experience_text, update_health_bar,
    update_health_text, update_level_text,
};
use systems::animation::animate_sprites;
use systems::combat::attack::monster_attack_system;
use systems::game_over::setup_game_over;
use systems::loading::check_assets_loaded;
use systems::monster::{generate_frames, monster_respawn_system};
use systems::monster_movement::monster_movement_system;
use systems::monster_state_system::monster_state_system;
use systems::{
    combat_system::combat_system, interaction::npc_interaction,
    movement::player_movement,
};

// Use resources
use resources::game_assets::GameAssets;

// Define GameState
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
    GameOver,
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
        .init_state::<GameState>()
        // .add_plugins(WorldInspectorPlugin::new())
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
                monster_attack_system,
                monster_respawn_system,
                animate_sprites,
                monster_movement_system,
                monster_state_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .run();
}

// System to setup entities once in the Playing state
fn setup(mut commands: Commands, assets: Res<GameAssets>, images: Res<Assets<Image>>) {
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

    // Set up monster animation frames
    let frame_size = Vec2::new(500.0, 500.0); // Based on your sprite sheet frame size
    let texture_size = {
        if let Some(image) = images.get(&assets.monster_sprite_sheet) {
            image.size().as_vec2()
        } else {
            Vec2::new(1000.0, 500.0) // Default size, adjust as needed
        }
    };
    let columns = 2; // Number of frames in a row
    let rows = 1; // Number of rows in the sprite sheet

    // Calculate scales
    let player_scale = calculate_scale(&assets.player, &images, None);
    let npc_scale = calculate_scale(&assets.npc, &images, None);

    let monster_scale =
        calculate_scale(&assets.monster_sprite_sheet, &images, Some(frame_size));

    // Set the timer duration (e.g., change direction every 2 to 5 seconds)
    let mut rng = thread_rng();
    let timer_duration = rng.gen_range(2.0..5.0);

    // Generate a random movement direction
    let direction =
        Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();

    // Generate frames for animation
    let frames = generate_frames(texture_size, frame_size, columns, rows);

    // Set speed based on monster type
    let idle_speed = 5.0;
    let aggressive_speed = 10.0;

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
        .spawn(SpriteBundle {
            texture: assets.monster_sprite_sheet.clone(),
            transform: Transform {
                translation: snap_to_grid(Vec3::new(300.0, 0.0, 0.0)),
                scale: monster_scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Monster)
        .insert(Stats::monster_stats(MonsterType::Lesser))
        .insert(MonsterMovement {
            direction,
            idle_speed,
            aggressive_speed,
        })
        .insert(AnimationIndices {
            frames: frames.clone(),
            current_frame: 0,
        })
        .insert(AnimationTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .insert(AttackTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert(MovementTimer(Timer::from_seconds(
            timer_duration,
            TimerMode::Repeating,
        )))
        .insert(MonsterState::Idle)
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
                        background_color: BackgroundColor(Color::Srgba(DARK_GRAY)),
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
                            background_color: BackgroundColor(Color::Srgba(DARK_GRAY)),
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    let zoom_speed = 0.1; // Adjust zoom speed as needed

    for mut ortho in query.iter_mut() {
        // Zoom using the scroll wheel
        for event in scroll_evr.read() {
            ortho.scale -= event.y * zoom_speed;
            ortho.scale = ortho.scale.clamp(0.4, 0.6); // Limit the zoom levels
        }

        // Zoom using keys (e.g., Z to zoom in, X to zoom out)
        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale -= zoom_speed;
            ortho.scale = ortho.scale.clamp(0.4, 0.6);
        }
        if keyboard_input.pressed(KeyCode::KeyX) {
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
fn load_game_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    // println!("Loading game assets...");
    let game_assets = GameAssets::new(&asset_server);
    commands.insert_resource(game_assets);
    // println!("Game assets resource inserted.");
}
