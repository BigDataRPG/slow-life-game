use crate::components::{monster::Monster, player::Player, stats::Stats};
use crate::systems::combat::attack::player_attack_system;
use crate::GameState;
use bevy::prelude::*;

pub fn combat_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Stats), (With<Player>, Without<Monster>)>,
    mut monster_query: Query<
        (Entity, &Transform, &mut Stats),
        (With<Monster>, Without<Player>),
    >,
    mut commands: Commands,
) {
    let (player_transform, mut player_stats) = match player_query.get_single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };

    if keyboard_input.just_pressed(KeyCode::KeyJ) {
        for (monster_entity, monster_transform, mut monster_stats) in
            monster_query.iter_mut()
        {
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

fn check_player_hp(
    query: Query<&Stats, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(stats) = query.get_single() {
        if stats.health <= 0 {
            // Transition to GameOver state if HP reaches 0
            next_state.set(GameState::GameOver);
        }
    }
}
