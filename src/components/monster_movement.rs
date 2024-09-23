use bevy::prelude::*;

#[derive(Component)]
pub struct MonsterMovement {
    pub direction: Vec3,
    pub speed: f32,
}
