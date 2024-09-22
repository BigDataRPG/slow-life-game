use bevy::prelude::*;

pub const GRID_SIZE: f32 = 32.0;

pub fn snap_to_grid(pos: Vec3) -> Vec3 {
    Vec3::new(
        (pos.x / GRID_SIZE).round() * GRID_SIZE,
        (pos.y / GRID_SIZE).round() * GRID_SIZE,
        pos.z,
    )
}

pub fn calculate_scale(texture_handle: &Handle<Image>, images: &Res<Assets<Image>>) -> Vec3 {
    if let Some(texture) = images.get(texture_handle) {
        let (width, height) = (texture.size().x, texture.size().y);
        Vec3::new(GRID_SIZE / width, GRID_SIZE / height, 1.0)
    } else {
        Vec3::ONE // Default scale if texture isn't loaded yet
    }
}
