use bevy::prelude::*;

pub const GRID_SIZE:  = 32.0;
pub const CHARACTER_SIZE_COEF: f32 = 1.5;

pub fn snap_to_grid(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x / GRID_SIZE).round() * GRID_SIZE,
        (pos.y / GRID_SIZE).round() * GRID_SIZE,
        pos.z,
    )
}

/// Calculates the scale for a sprite based on its frame size from a TextureAtlas.
pub fn calculate_scale_atlas(
    texture_atlas_handle: &Handle<TextureAtlas>,
    texture_atlases: &Res<Assets<TextureAtlas>>,
) -> Vec3 {
    if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
        // Assuming each frame should fit within GRID_SIZE * CHARACTER_SIZE_COEF
        let frame_size = texture_atlas.size;
        Vec3::new(
            GRID_SIZE * CHARACTER_SIZE_COEF / frame_size.x,
            GRID_SIZE / frame_size.y,
            1.0,
        )
    } else {
        Vec3::ONE // Default scale if TextureAtlas isn't loaded yet
    }
}

pub fn calculate_scale(texture_handle: &Handle<Image>, images: &Assets<Image>) -> Vec3 {
    if let Some(texture) = images.get(texture_handle) {
        let width = texture.size().x;
        let height = texture.size().y;
        // Calculate the desired scale based on your requirements
        // For example, scale to 64x64 units
        Vec3::new(GRID_SIZE / width, GRID_SIZE / height, 1.0)
    } else {
        Vec3::ONE
    }
}

