use crate::components::stats::Stats;
use rand::Rng;

pub fn apply_damage_to_player(player_stats: &mut Stats, damage: i32) {
    player_stats.health -= damage.max(1);
    println!("Player received {} damage", damage.max(1));
    if player_stats.health <= 0 {
        println!("Player defeated!");
        player_stats.health = player_stats.max_health;
    }
}

pub fn apply_damage_to_monster(monster_stats: &mut Stats, damage: i32) {
    monster_stats.health -= damage;
    println!("Monster hit! Health is now {}", monster_stats.health);
    if monster_stats.health <= 0 {
        println!("Monster defeated!");
    }
}

pub fn monster_retaliate(
    rng: &mut rand::rngs::ThreadRng,
    player_stats: &mut Stats,
    monster_stats: &mut Stats,
) {
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
