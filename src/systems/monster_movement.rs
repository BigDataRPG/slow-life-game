use bevy::prelude::*;
use rand::prelude::*;

use crate::components::timer_component::MovementTimer;
use crate::components::{monster::Monster, monster_movement::MonsterMovement};

pub fn monster_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut MonsterMovement, &mut MovementTimer), With<Monster>>,
) {
    // Define your game's boundary
    let boundary = 250.0;
    let mut rng = thread_rng();

    for (mut transform, mut monster_movement, mut timer) in query.iter_mut() {
        transform.translation +=
            monster_movement.direction * monster_movement.speed * time.delta_seconds();

        timer.tick(time.delta());
        if timer.finished() {
            // Change direction randomly every few seconds
            monster_movement.direction =
                Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
        }

        // Check for boundaries and invert direction if necessary
        if transform.translation.x.abs() > boundary {
            monster_movement.direction.x = -monster_movement.direction.x;
        }
        if transform.translation.y.abs() > boundary {
            monster_movement.direction.y = -monster_movement.direction.y;
        }
    }
}
