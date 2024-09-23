use crate::components::{monster::Monster, monster_state::MonsterState, player::Player};
use crate::resources::physic::*;
use bevy::prelude::*;

pub fn monster_state_system(
    mut query: Query<(&Transform, &mut MonsterState, Entity), With<Monster>>,
    player_query: Query<&Transform, (With<Player>, Without<Monster>)>,
    mut sprite_query: Query<&mut Sprite, (With<Monster>, Without<Player>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for (monster_transform, mut state, monster_entity) in query.iter_mut() {
        let distance = monster_transform
            .translation
            .distance(player_transform.translation);

        if *state == MonsterState::Idle && distance <= MONSTER_DETECTION_RANGE {
            // Switch to Aggressive state
            *state = MonsterState::Aggressive;
            // Change monster color to red
            if let Ok(mut sprite) = sprite_query.get_mut(monster_entity) {
                sprite.color = Color::RED;
            }
            println!("Monster became aggressive!");
        } else if *state == MonsterState::Aggressive && distance > MONSTER_DETECTION_RANGE {
            // Switch back to Idle state
            *state = MonsterState::Idle;
            // Change monster color back to default
            if let Ok(mut sprite) = sprite_query.get_mut(monster_entity) {
                sprite.color = Color::WHITE;
            }
            println!("Monster calmed down.");
        }
    }
}
