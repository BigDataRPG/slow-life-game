use bevy::prelude::*;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct NPC;

const GRID_SIZE: f32 = 64.0; // Adjust based on your map's grid size

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, images: Res<Assets<Image>>) {
    // Add a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background_test.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0), // Behind the player
            ..Default::default()
        },
        ..Default::default()
    });

    // Load textures
    let player_texture = asset_server.load("player_test.png");
    let npc_texture = asset_server.load("npc_test.png");

    // Calculate scales
    let player_scale = calculate_scale(&player_texture, &images);
    let npc_scale = calculate_scale(&npc_texture, &images);

    // Spawn the player
    commands
        .spawn(SpriteBundle {
            texture: player_texture.clone(),
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
            texture: npc_texture.clone(),
            transform: Transform {
                translation: snap_to_grid(Vec3::new(200.0, 0.0, 0.0)),
                scale: npc_scale,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NPC);
}

// Marker component for the player
fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
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
            let speed = 200.0;
            transform.translation += direction.normalize() * speed * time.delta_seconds();

            // Snap to grid after movement
            transform.translation = snap_to_grid(transform.translation);
        }
    }
}

fn npc_interaction(
    player_query: Query<&Transform, With<Player>>,
    npc_query: Query<(Entity, &Transform), With<NPC>>,
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
            // Interaction distance based on grid size
            println!("You are interacting with an NPC!");

            // Optional: Remove NPC after interaction
            // commands.entity(npc_entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .add_systems(Update, npc_interaction)
        .run();
}
