use bevy::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    pub damage: i32,
    pub owner: Entity,
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct AttackHitbox {
    pub damage: i32,
    pub owner: Entity,
    pub lifetime: Timer,
}
