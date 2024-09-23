use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub player: Handle<Image>,
    pub npc: Handle<Image>,
    pub mask: Handle<Image>,

    // Texture Atlas for the Monster
    pub monster_sprite_sheet: Handle<TextureAtlasLayout>,
}

impl GameAssets {
    /// Loads all necessary game assets, including the monster sprite sheet.
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        // Load individual images
        let background = asset_server.load("images/background/background_test.png");
        let player = asset_server.load("images/players/player_test.png");
        let npc = asset_server.load("images/npc/npc_test.png");
        let mask = asset_server.load("images/background/mask_test.png");

        // Load the monster sprite sheet image
        let monster_sprite_sheet = asset_server.load("images/monsters/monster_sprite_sheet.png");

        // the sprite sheet has 2 sprites arranged in a row, and they are all 500px x 500px
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(500), 2, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        GameAssets {
            background,
            player,
            npc,
            mask,
            monster_sprite_sheet,
        }
    }
}
