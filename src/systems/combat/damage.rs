use crate::components::stats::Stats;

pub fn apply_damage_to_player(player_stats: &mut Stats, damage: i32) -> bool {
    let actual_damage = damage.max(1);
    player_stats.health -= actual_damage;
    println!("Player received {} damage", actual_damage);

    if player_stats.health <= 0 {
        println!("Player defeated!");
        true // Return true indicating the player is defeated
    } else {
        false
    }
}
