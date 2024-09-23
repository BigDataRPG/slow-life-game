use crate::components::hitbox::Hitbox;
use crate::components::stats::Stats;
use crate::systems::combat::damage::{apply_damage_to_monster, apply_damage_to_player};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn hitbox_collision_system(
    mut commands: Commands,
    mut hitbox_query: Query<(Entity, &Hitbox, &Collider, &GlobalTransform)>,
    mut player_query: Query<(Entity, &mut Stats, &Collider, &GlobalTransform), With<Player>>,
    mut monster_query: Query<(Entity, &mut Stats, &Collider, &GlobalTransform), With<Monster>>,
    rapier_context: Res<RapierContext>,
) {
    for (hitbox_entity, hitbox, hitbox_collider, hitbox_transform) in hitbox_query.iter_mut() {
        for (monster_entity, mut monster_stats, monster_collider, monster_transform) in
            monster_query.iter_mut()
        {
            let collision = rapier_context.intersection_pair(hitbox_entity, monster_entity);
            if let Some(true) = collision {
                apply_damage_to_monster(&mut monster_stats, hitbox.damage);
                commands.entity(hitbox_entity).despawn();
                if monster_stats.health <= 0 {
                    commands.entity(monster_entity).despawn();
                }
            }
        }

        for (player_entity, mut player_stats, player_collider, player_transform) in
            player_query.iter_mut()
        {
            if hitbox.owner != player_entity {
                let collision = rapier_context.intersection_pair(hitbox_entity, player_entity);
                if let Some(true) = collision {
                    apply_damage_to_player(&mut player_stats, hitbox.damage);
                    commands.entity(hitbox_entity).despawn();
                }
            }
        }
    }
}

pub fn hitbox_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut hitbox_query: Query<(Entity, &mut Hitbox)>,
) {
    for (entity, mut hitbox) in hitbox_query.iter_mut() {
        hitbox.lifetime.tick(time.delta());
        if hitbox.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
