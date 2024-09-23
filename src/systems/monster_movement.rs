use bevy::prelude::*;
use rand::prelude::*;

use crate::components::player::Player;
use crate::components::timer_component::MovementTimer;
use crate::components::{
    monster::Monster, monster_movement::MonsterMovement, monster_state::MonsterState,
};

pub fn monster_movement_system(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut MonsterMovement,
            &mut MovementTimer,
            &MonsterState,
        ),
        (With<Monster>, Without<Player>),
    >,
    player_query: Query<&mut Transform, (With<Player>, Without<Monster>)>,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Define your game's boundary
    let boundary = 250.0;
    let mut rng = thread_rng();

    for (mut transform, mut monster_movement, mut movement_timer, monster_state) in query.iter_mut()
    {
        match *monster_state {
            MonsterState::Idle => {
                movement_timer.tick(time.delta());
                if movement_timer.finished() {
                    let mut rng = rand::thread_rng();
                    monster_movement.direction =
                        Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
                            .normalize();
                    // Check for boundaries and invert direction if necessary
                    if transform.translation.x.abs() > boundary {
                        monster_movement.direction.x = -monster_movement.direction.x;
                    }
                    if transform.translation.y.abs() > boundary {
                        monster_movement.direction.y = -monster_movement.direction.y;
                    }
                    movement_timer.reset();
                }
                transform.translation +=
                    monster_movement.direction * monster_movement.idle_speed * time.delta_seconds();
            }
            MonsterState::Aggressive => {
                // Aggressive behavior (follow player)
                let direction =
                    (player_transform.translation - transform.translation).normalize_or_zero();
                transform.translation +=
                    direction * monster_movement.aggressive_speed * time.delta_seconds();
            }
        }
    }
}
