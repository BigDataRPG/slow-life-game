use crate::components::{monster::Monster, player::Player, stats::Stats};
use crate::systems::combat::attack::player_attack_system;
use bevy::prelude::*;

pub fn combat_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Stats), (With<Player>, Without<Monster>)>,
    mut monster_query: Query<(Entity, &Transform, &mut Stats), (With<Monster>, Without<Player>)>,
    mut commands: Commands,
) {
    let (player_transform, mut player_stats) = match player_query.get_single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };

    if keyboard_input.just_pressed(KeyCode::J) {
        for (monster_entity, monster_transform, mut monster_stats) in monster_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(monster_transform.translation);

            if distance < 32.0 {
                // Assuming GRID_SIZE is 32.0
                player_attack_system(
                    &mut commands,
                    &mut player_stats,
                    monster_entity,
                    &mut monster_stats,
                );

                break; // Only attack one monster at a time
            }
        }
    }
}
