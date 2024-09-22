use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub player: Handle<Image>,
    pub npc: Handle<Image>,
    pub mask: Handle<Image>,
    pub monster: Handle<Image>,
}
