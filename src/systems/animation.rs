use bevy::prelude::*;

use crate::components::animation::{AnimationIndices, AnimationTimer};
use bevy::sprite::TextureAtlasSprite;

/// System to animate sprites by cycling through TextureAtlas frames.
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimationIndices,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, animation_indices, mut sprite) in query.iter_mut() {
        // Tick the timer
        timer.tick(time.delta());

        // Check if the timer has finished
        if timer.just_finished() {
            // Advance to the next frame
            if sprite.index == animation_indices.last {
                sprite.index = animation_indices.first; // Loop back to the first frame
            } else {
                sprite.index += 1; // Move to the next frame
            }
        }
    }
}
