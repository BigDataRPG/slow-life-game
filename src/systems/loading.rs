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
    let handles: Vec<UntypedHandle> = vec![
        assets.background.is_weak(),
        assets.player.is_weak(),
        assets.npc.is_weak(),
        assets.mask.is_weak(),
        assets.monster_sprite_sheet_image.is_weak(), // Use the image handle
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
