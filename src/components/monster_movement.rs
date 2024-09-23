use bevy::prelude::*;

#[derive(Component)]
pub struct MonsterMovement {
    pub direction: Vec3,
    pub idle_speed: f32,
    pub aggressive_speed: f32,
}
