use bevy::prelude::*;

use crate::components::animation::{AnimationIndices, AnimationTimer};

/// System to animate sprites by cycling through frames using the `rect` field.
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut AnimationIndices, &mut Sprite)>,
) {
    for (mut timer, mut animation_indices, mut sprite) in query.iter_mut() {
        // Tick the timer
        timer.tick(time.delta());

        // Check if the timer has finished
        if timer.just_finished() {
            // Advance to the next frame
            animation_indices.current_frame =
                (animation_indices.current_frame + 1) % animation_indices.frames.len();

            // Update the sprite's rect to display the current frame
            sprite.rect =
                Some(animation_indices.frames[animation_indices.current_frame]);
        }
    }
}

// Function to calculate the frame rectangles
fn _generate_frames(
    _texture_size: Vec2,
    frame_size: Vec2,
    columns: usize,
    rows: usize,
) -> Vec<Rect> {
    let mut frames = Vec::new();
    for row in 0..rows {
        for column in 0..columns {
            let min =
                Vec2::new(column as f32 * frame_size.x, row as f32 * frame_size.y);
            let max = min + frame_size;
            frames.push(Rect { min, max });
        }
    }
    frames
}
