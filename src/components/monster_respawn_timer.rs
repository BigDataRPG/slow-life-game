use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct MonsterRespawnTimer(pub Timer);
