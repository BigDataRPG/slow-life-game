use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub background: Handle<Image>,
    pub player: Handle<Image>,
    pub npc: Handle<Image>,
    pub mask: Handle<Image>,

    // Texture Atlas for the Monster
    pub monster_sprite_sheet: Handle<TextureAtlas>,
    pub monster_sprite_sheet_image: Handle<Image>, // Add this line
}

impl GameAssets {
    /// Loads all necessary game assets, including the monster sprite sheet.
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        // Load individual images
        let background = asset_server.load("images/background/background_test.png");
        let player = asset_server.load("images/players/player_test.png");
        let npc = asset_server.load("images/npc/npc_test.png");
        let mask = asset_server.load("images/background/mask_test.png");

        // Load the monster sprite sheet image
        let monster_sprite_sheet_image =
            asset_server.load("images/monsters/monster_sprite_sheet.png");

        // Define the frame size (500x500 pixels)
        let frame_size = Vec2::new(500.0, 500.0);

        // Create the TextureAtlas from the sprite sheet image
        let monster_sprite_sheet = texture_atlases.add(TextureAtlas::from_grid(
            monster_sprite_sheet_image.clone(), // Use the image handle here
            frame_size,
            2,    // Number of columns
            1,    // Number of rows
            None, // No spacing between sprites
            None, // No padding
        ));

        GameAssets {
            background,
            player,
            npc,
            mask,
            monster_sprite_sheet,
            monster_sprite_sheet_image, // Add this line
        }
    }
}
