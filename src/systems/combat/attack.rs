use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{
    monster::Monster, monster_state::MonsterState, player::Player, stats::Stats,
    timer_component::AttackTimer,
};

use crate::systems::combat::damage::apply_damage_to_player;

use crate::resources::physic::*;

use crate::utils::combat_utils::is_critical_hit;

pub fn player_attack_system(
    commands: &mut Commands,
    player_stats: &mut Stats,
    monster_entity: Entity,
    monster_stats: &mut Stats,
) {
    let mut rng = rand::thread_rng();

    // Determine if the player's attack hits the monster
    if rng.gen::<f32>() < player_stats.hit_rate && rng.gen::<f32>() > monster_stats.free_rate {
        // Determine if it's a critical hit
        let is_critical = is_critical_hit(&mut rng, player_stats.critical_rate);
        let base_damage = player_stats.attack - monster_stats.defense;
        let damage_to_monster = if is_critical {
            (base_damage as f32 * player_stats.critical_damage).round() as i32
        } else {
            base_damage.max(1)
        };

        monster_stats.health -= damage_to_monster;
        if is_critical {
            println!(
                "Critical hit! You dealt {} damage to the monster!",
                damage_to_monster
            );
        } else {
            println!("You attacked the monster for {} damage!", damage_to_monster);
        }

        if monster_stats.health <= 0 {
            println!("Monster defeated!");
            player_stats.gain_experience(100);
            commands.entity(monster_entity).despawn();
        }
    } else {
        println!("Your attack missed the monster!");
    }
}

pub fn monster_attack_system(
    mut commands: Commands,
    time: Res<Time>,
    mut monster_query: Query<
        (Entity, &Transform, &Stats, &mut AttackTimer, &MonsterState),
        (With<Monster>, Without<Player>),
    >,
    mut player_query: Query<(&Transform, &mut Stats), (With<Player>, Without<Monster>)>,
) {
    let (player_transform, mut player_stats) = match player_query.get_single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };

    for (_monster_entity, monster_transform, monster_stats, mut attack_timer, state) in
        monster_query.iter_mut()
    {
        if *state != MonsterState::Aggressive {
            continue; // Skip if not aggressive
        }

        attack_timer.tick(time.delta());
        if attack_timer.finished() {
            let distance = monster_transform
                .translation
                .distance(player_transform.translation);

            if distance <= MONSTER_ATTACK_RANGE {
                // Monster attacks the player
                monster_attack_player(&mut commands, monster_stats, &mut player_stats);
            }

            // Reset the timer for the next attack
            attack_timer.reset();
        }
    }
}

pub fn monster_attack_player(
    _commands: &mut Commands,
    monster_stats: &Stats,
    player_stats: &mut Stats,
) {
    let mut rng = rand::thread_rng();

    if rng.gen::<f32>() < monster_stats.hit_rate {
        let is_critical = rng.gen::<f32>() < monster_stats.critical_rate;
        let base_damage = monster_stats.attack - player_stats.defense;
        let damage = if is_critical {
            (base_damage as f32 * monster_stats.critical_damage).round() as i32
        } else {
            base_damage.max(1)
        };

        apply_damage_to_player(player_stats, damage);
    } else {
        println!("The monster's attack missed you!");
    }
}
