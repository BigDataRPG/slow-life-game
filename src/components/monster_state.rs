use bevy::prelude::*;

#[derive(Component, PartialEq, Eq)]
pub enum MonsterState {
    Idle,
    Aggressive,
}
