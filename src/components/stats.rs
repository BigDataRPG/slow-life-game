use bevy::prelude::*;

use crate::components::monster_type::MonsterType;

#[derive(Component)]
pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub experience: i32,
    pub level: i32,
    pub critical_rate: f32, // Percentage chance to land a critical hit (e.g., 0.1 for 10%)
    pub critical_damage: f32, // Multiplier for critical hits (e.g., 1.5 for 150% damage)
    pub hit_rate: f32,      // Percentage chance to hit the target (e.g., 0.9 for 90%)
    pub free_rate: f32,     // Custom stat, could be used for dodge or some other mechanic
}

impl Stats {
    // Player stats factory method
    pub fn player_stats(health: i32, attack: i32, defense: i32) -> Self {
        Stats {
            health,
            max_health: health,
            attack,
            defense,
            experience: 0,
            level: 1,
            critical_rate: 0.1,   // Default critical rate for player
            critical_damage: 1.5, // Default critical damage for player
            hit_rate: 0.9,        // Default hit rate for player
            free_rate: 0.05,      // Default free rate for player
        }
    }

    // Monster stats factory method based on the type of monster
    pub fn monster_stats(monster_type: MonsterType) -> Self {
        match monster_type {
            MonsterType::Lesser => Stats {
                health: 30,
                max_health: 30,
                attack: 5,
                defense: 2,
                experience: 0,
                level: 1,
                critical_rate: 0.05,  // Lesser monsters have lower critical rates
                critical_damage: 1.2, // Lesser monsters have lower critical damage
                hit_rate: 0.8,        // Lesser monsters have lower hit rate
                free_rate: 0.02,      // Custom dodge rate
            },
            MonsterType::Elite => Stats {
                health: 100,
                max_health: 100,
                attack: 15,
                defense: 10,
                experience: 0,
                level: 1,
                critical_rate: 0.15,  // Elite monsters have better critical rates
                critical_damage: 1.7, // Elite monsters have stronger critical damage
                hit_rate: 0.85,       // Elite monsters have better hit rate
                free_rate: 0.05,      // Custom dodge rate
            },
            MonsterType::King => Stats {
                health: 200,
                max_health: 200,
                attack: 25,
                defense: 20,
                experience: 0,
                level: 1,
                critical_rate: 0.2,   // King monsters have higher critical rates
                critical_damage: 2.0, // King monsters have stronger critical damage
                hit_rate: 0.9,        // King monsters have a high hit rate
                free_rate: 0.1,       // Custom dodge rate
            },
            MonsterType::Legend => Stats {
                health: 500,
                max_health: 500,
                attack: 50,
                defense: 40,
                experience: 0,
                level: 1,
                critical_rate: 0.3, // Legendary monsters have high critical rates
                critical_damage: 2.5, // Legendary monsters deal massive critical damage
                hit_rate: 0.95,     // Legendary monsters rarely miss
                free_rate: 0.15,    // Custom dodge rate
            },
        }
    }

    // Common method to handle gaining experience
    pub fn gain_experience(&mut self, amount: i32) {
        self.experience += amount;
        // Loop to handle multiple level-ups
        while self.experience >= self.level * 100 {
            self.experience -= self.level * 100;
            self.level_up();
        }
    }

    // Common method to handle leveling up
    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 10;
        self.attack += 2;
        self.defense += 2;
        self.health = self.max_health;
        self.critical_rate += 0.01; // Increase critical rate by 1% each level
        self.critical_damage += 0.05; // Increase critical damage multiplier
        self.hit_rate += 0.01; // Increase hit rate by 1% each level
        self.free_rate += 0.01; // Increase custom stat
        println!("Level up! You are now level {}!", self.level);
    }

    // Method to determine experience needed for the next level
    pub fn exp_next_level(&self) -> i32 {
        self.level * 100 // Example formula: 100 EXP per level
    }
}
