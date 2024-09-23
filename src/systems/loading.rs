use crate::resources::game_assets::GameAssets;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub fn check_assets_loaded(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // println!("Checking if assets are loaded...");

    // Collect all asset handles into a vector of untyped handles
    let handles: Vec<HandleUntyped> = vec![
        assets.background.clone_untyped(),
        assets.player.clone_untyped(),
        assets.npc.clone_untyped(),
        assets.mask.clone_untyped(),
        assets.monster_sprite_sheet_image.clone_untyped(), // Use the image handle
    ];

    let mut all_loaded = true;

    for handle in &handles {
        let load_state = asset_server.get_load_state(handle.id());
        // println!("Asset {:?} load state: {:?}", handle.id(), load_state);

        if load_state == LoadState::Failed {
            // println!("Failed to load asset: {:?}", handle.id());
            all_loaded = false;
            break;
        } else if load_state != LoadState::Loaded {
            all_loaded = false;
            break;
        }
    }

    if all_loaded {
        // println!("All assets loaded, transitioning to Playing state.");
        next_state.set(GameState::Playing);
    } else {
        // println!("Assets are still loading.");
    }
}
