use bevy::prelude::*;

pub const GRID_SIZE: f32 = 32.0;
pub const CHARACTER_SIZE_COEF: f32 = 1.5;

pub fn snap_to_grid(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x / GRID_SIZE).round() * GRID_SIZE,
        (pos.y / GRID_SIZE).round() * GRID_SIZE,
        pos.z,
    )
}

pub fn calculate_scale(
    texture_handle: &Handle<Image>,
    images: &Assets<Image>,
    frame_size: Option<Vec2>,
) -> Vec3 {
    if let Some(texture) = images.get(texture_handle) {
        let (width, height) = if let Some(frame) = frame_size {
            (frame.x, frame.y)
        } else {
            (texture.size().x as f32, texture.size().y as f32)
        };
        Vec3::new(
            GRID_SIZE * CHARACTER_SIZE_COEF / width,
            GRID_SIZE * CHARACTER_SIZE_COEF / height,
            1.0,
        )
    } else {
        Vec3::ONE
    }
}
