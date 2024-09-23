use crate::components::hitbox::Hitbox;
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::components::stats::Stats;
use crate::components::timer_component::AttackTimer;
use crate::resources::physic::*;
use crate::utils::combat_utils::is_critical_hit;

use bevy::prelude::*;

use rand::Rng;

// Import Timer and TimerMode
use bevy::prelude::{Timer, TimerMode};

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
    mut monster_query: Query<(Entity, &Transform, &Stats, &mut AttackTimer), With<Monster>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for (monster_entity, monster_transform, monster_stats, mut attack_timer) in
        monster_query.iter_mut()
    {
        attack_timer.0.tick(time.delta());
        if attack_timer.0.finished() {
            // Calculate direction towards the player
            let direction =
                (player_transform.translation - monster_transform.translation).normalize_or_zero();

            // Calculate the attack position
            let attack_position = monster_transform.translation + direction * HITBOX_RANGE;

            // Create the monster's attack hitbox
            commands.spawn((
                Transform::from_translation(attack_position),
                GlobalTransform::default(),
                // Collider::cuboid(MONSTER_HITBOX_SIZE.0, MONSTER_HITBOX_SIZE.1),
                // Sensor,
                // ActiveEvents::COLLISION_EVENTS,
                Hitbox {
                    damage: monster_stats.attack,
                    owner: monster_entity,
                    lifetime: Timer::from_seconds(HITBOX_LIFETIME, TimerMode::Once),
                },
            ));
        }
    }
}
