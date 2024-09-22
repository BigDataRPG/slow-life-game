use bevy::prelude::*;

#[derive(Component)]
pub struct Stats {
    pub health: i32,
    pub max_health: i32,
    pub attack: i32,
    pub defense: i32,
    pub experience: i32,
    pub level: i32,
}

impl Stats {
    pub fn new(health: i32, attack: i32, defense: i32) -> Self {
        Stats {
            health,
            max_health: health,
            attack,
            defense,
            experience: 0,
            level: 1,
        }
    }

    pub fn gain_experience(&mut self, amount: i32) {
        self.experience += amount;
        let exp_needed = self.level * 100; // Experience needed to level up
        if self.experience >= exp_needed {
            self.experience -= exp_needed;
            self.level_up();
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_health += 10;
        self.attack += 2;
        self.defense += 2;
        self.health = self.max_health;
        println!("Level up! You are now level {}!", self.level);
    }
}
