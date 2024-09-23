use crate::components::monster_type::MonsterType;
use crate::components::{monster::Monster, monster_state::MonsterState, player::Player};
use crate::resources::physic::*;
use bevy::prelude::*;

pub fn monster_state_system(
    mut query: Query<
        (
            &Transform,
            &mut MonsterState,
            &mut TextureAtlasSprite,
            &MonsterType,
        ),
        With<Monster>,
    >,
    player_query: Query<&Transform, (With<Player>, Without<Monster>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for (monster_transform, mut state, mut sprite, monster_type) in query.iter_mut() {
        let distance = monster_transform
            .translation
            .distance(player_transform.translation);

        if *state == MonsterState::Idle && distance <= MONSTER_DETECTION_RANGE {
            // Switch to Aggressive state
            *state = MonsterState::Aggressive;
            sprite.color = Color::RED;
            println!("Monster became aggressive!");
        } else if *state == MonsterState::Aggressive && distance > MONSTER_DETECTION_RANGE {
            // Switch back to Idle state
            *state = MonsterState::Idle;
            sprite.color = monster_type.color(); // Reset to default color
            println!("Monster calmed down.");
        }
    }
}
