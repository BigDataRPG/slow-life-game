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
        assets.background.clone().into(),
        assets.player.clone().into(),
        assets.npc.clone().into(),
        assets.mask.clone().into(),
        assets.monster_sprite_sheet.clone().into(), // Use the image handle
    ];

    let mut all_loaded = true;

    for handle in &handles {
        let load_state = asset_server.get_load_state(handle.id());
        // println!("Asset {:?} load state: {:?}", handle.id(), load_state);

        // Handle the case where load_state is an Option<LoadState>
        match load_state {
            Some(LoadState::Failed(_)) => {
                // Asset loading failed
                all_loaded = false;
                break;
            }
            Some(LoadState::Loaded) => {
                // Asset successfully loaded, continue checking others
            }
            Some(_) | None => {
                // Still loading or unknown state, treat as not fully loaded
                all_loaded = false;
                break;
            }
        }
    }

    if all_loaded {
        // println!("All assets loaded, transitioning to Playing state.");
        next_state.set(GameState::Playing);
    } else {
        // println!("Assets are still loading.");
    }
}
