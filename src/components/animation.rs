use bevy::prelude::*;

/// Specifies the range of frames in the animation.
#[derive(Component)]
pub struct AnimationIndices {
    pub frames: Vec<Rect>,
    pub current_frame: usize,
}

/// Timer to control animation frame updates.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
