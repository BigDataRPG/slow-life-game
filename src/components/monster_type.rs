use bevy::color::palettes::css::*;
use bevy::prelude::Color::Srgba;
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
            MonsterType::Lesser => Srgba(GREEN),
            MonsterType::Elite => Srgba(YELLOW),
            MonsterType::King => Srgba(BLUE_VIOLET),
            MonsterType::Legend => Srgba(BLACK),
        }
    }
}
