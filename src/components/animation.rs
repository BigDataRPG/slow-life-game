use bevy::prelude::*;

/// Specifies the range of frames in the animation.
#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize, // Starting frame index
    pub last: usize,  // Ending frame index
}

/// Timer to control animation frame updates.
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
