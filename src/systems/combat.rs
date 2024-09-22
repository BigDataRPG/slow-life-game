use crate::components::{monster::Monster, player::Player, stats::Stats};
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

    if keyboard_input.just_pressed(KeyCode::F) {
        for (monster_entity, monster_transform, mut monster_stats) in monster_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(monster_transform.translation);

            if distance < 32.0 {
                // Assuming GRID_SIZE is 32.0
                let damage_to_monster = (player_stats.attack - monster_stats.defense).max(1);
                monster_stats.health -= damage_to_monster;
                println!("You attacked the monster for {} damage!", damage_to_monster);

                if monster_stats.health <= 0 {
                    println!("Monster defeated!");
                    player_stats.gain_experience(100);
                    commands.entity(monster_entity).despawn();
                } else {
                    let damage_to_player = (monster_stats.attack - player_stats.defense).max(1);
                    player_stats.health -= damage_to_player;
                    println!(
                        "Monster retaliated and dealt {} damage to you!",
                        damage_to_player
                    );

                    if player_stats.health <= 0 {
                        println!("You have been defeated!");
                        player_stats.health = player_stats.max_health; // Reset health for simplicity
                    }
                }
                break; // Only attack one monster at a time
            }
        }
    }
}
