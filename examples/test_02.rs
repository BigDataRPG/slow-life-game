use bevy::asset::LoadState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

// Define and derive States for GameState
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    Loading,
    Playing,
}

// Manually implement Default for GameState
impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct NPC;

#[derive(Resource)]
struct GameAssets {
    background: Handle<Image>,
    player: Handle<Image>,
    npc: Handle<Image>,
}

const GRID_SIZE: f32 = 32.0;

// Function to snap a position to the nearest grid point
fn snap_to_grid(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x / GRID_SIZE).round() * GRID_SIZE,
        (pos.y / GRID_SIZE).round() * GRID_SIZE,
        pos.z,
    )
}

// Function to calculate scale based on texture size and grid size
fn calculate_scale(texture_handle: &Handle<Image>, images: &Res<Assets<Image>>) -> Vec3 {
    if let Some(texture) = images.get(texture_handle) {
        let (width, height) = (texture.size().x, texture.size().y);
        Vec3::new(GRID_SIZE / width, GRID_SIZE / height, 1.0)
    } else {
        Vec3::ONE // Default scale if texture isn't loaded yet
    }
}

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
        .add_systems(OnEnter(GameState::Playing), setup)
        // Systems that run every frame while in the Playing state
        .add_systems(
            Update,
            (
                player_movement,
                npc_interaction,
                camera_zoom,
                camera_follow_player,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

// System to load assets during Loading state
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        background: asset_server.load("background_test.png"),
        player: asset_server.load("player_test.png"),
        npc: asset_server.load("npc_test.png"),
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

    if background_loaded && player_loaded && npc_loaded {
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
        .insert(Player);

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
}

// Player movement system: grid-based movement on key press
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 200.0; // Adjust the speed as needed
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            // Normalize the direction vector to have a consistent speed in all directions
            direction = direction.normalize();
            // Move the player
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}

// NPC interaction system
fn npc_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    npc_query: Query<(Entity, &Transform), With<NPC>>,
    mut _commands: Commands,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return, // Handle the case where there's not exactly one player
    };

    for (_npc_entity, npc_transform) in npc_query.iter() {
        let distance = player_transform
            .translation
            .distance(npc_transform.translation);

        if distance < GRID_SIZE / 2.0 {
            // Player is within interaction range
            if keyboard_input.just_pressed(KeyCode::Space) {
                // Player pressed the interaction button
                println!("You are Nooooob, get out of my way!");

                // Optional: Remove NPC after interaction
                // commands.entity(npc_entity).despawn();
            }
        }
    }
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
