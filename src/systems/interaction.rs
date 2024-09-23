use crate::components::npc::NPC;
use crate::components::{player::Player, stats::Stats};
use bevy::prelude::*;

pub fn npc_interaction(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &mut Stats), With<Player>>,
    npc_query: Query<&Transform, With<NPC>>,
) {
    let (player_transform, mut player_stats) = match player_query.get_single_mut() {
        Ok(result) => result,
        Err(_) => return,
    };

    for npc_transform in npc_query.iter() {
        let distance = player_transform
            .translation
            .distance(npc_transform.translation);

        if distance < 32.0 {
            // Assuming GRID_SIZE is 32.0
            if keyboard_input.just_pressed(KeyCode::Space) {
                println!("You are Nooooob, get out of my way!");
                player_stats.gain_experience(50000); // Gain experience
            }
        }
    }
}
