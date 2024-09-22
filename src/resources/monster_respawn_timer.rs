use bevy::prelude::*;

#[derive(Resource)]
pub struct MonsterRespawnTimer {
    pub timer: Timer,
    // Add any other fields if necessary
}
