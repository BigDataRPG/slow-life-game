use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum MonsterType {
    Lesser,
    Elite,
    King,
    Legend,
}

impl MonsterType {
    pub fn color(&self) -> Color {
        match self {
            MonsterType::Lesser => Color::GREEN,
            MonsterType::Elite => Color::YELLOW,
            MonsterType::King => Color::ORANGE,
            MonsterType::Legend => Color::PURPLE,
        }
    }
}
