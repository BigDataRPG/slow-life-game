use bevy::prelude::*;

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct MovementTimer(pub Timer);
