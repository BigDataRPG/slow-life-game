use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{
    animation::{AnimationIndices, AnimationTimer},
    monster::{Monster, MonsterHealthBar, MonsterHealthBarBackground},
    monster_movement::MonsterMovement,
    monster_respawn_timer::MonsterRespawnTimer,
    monster_state::MonsterState,
    stats::{MonsterType, Stats},
    timer_component::{AttackTimer, MovementTimer},
};
use crate::resources::game_assets::GameAssets;
use crate::utils::common::{calculate_scale_atlas, snap_to_grid};

pub fn monster_respawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MonsterRespawnTimer>,
    assets: Res<GameAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    query: Query<(), With<Monster>>, // To count existing monsters
) {
    timer.tick(time.delta());

    if timer.finished() && query.iter().count() < 20 {
        let spawn_area = -250.0..250.0;
        let mut rng = thread_rng();
        let x = rng.gen_range(spawn_area.clone());
        let y = rng.gen_range(spawn_area.clone());

        let weights = [50, 30, 15, 5]; // Lesser: 50%, Elite: 30%, King: 15%, Legend: 5%

        let roll = rng.gen_range(0..100);
        let monster_type = if roll < weights[0] {
            MonsterType::Lesser
        } else if roll < weights[0] + weights[1] {
            MonsterType::Elite
        } else if roll < weights[0] + weights[1] + weights[2] {
            MonsterType::King
        } else {
            MonsterType::Legend
        };

        let animation_indices = AnimationIndices { first: 0, last: 1 };
        let monster_scale = calculate_scale_atlas(&assets.monster_sprite_sheet, &texture_atlases);

        let monster_color = match monster_type {
            MonsterType::Lesser => Color::GREEN,
            MonsterType::Elite => Color::YELLOW,
            MonsterType::King => Color::ORANGE,
            MonsterType::Legend => Color::PURPLE,
        };

        // Generate a random movement direction
        let direction =
            Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();

        // Set speeds based on monster type
        let (idle_speed, aggressive_speed) = match monster_type {
            MonsterType::Lesser => (5.0, 10.0),
            MonsterType::Elite => (15.0, 30.0),
            MonsterType::King => (50.0, 75.0),
            MonsterType::Legend => (110.0, 150.0),
        };

        // Set the timer duration (e.g., change direction every 2 to 5 seconds)
        let timer_duration = rng.gen_range(2.0..5.0);

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: assets.monster_sprite_sheet.clone(),
                sprite: TextureAtlasSprite {
                    index: animation_indices.first,
                    color: monster_color,
                    ..Default::default()
                },
                transform: Transform {
                    translation: snap_to_grid(Vec3::new(x, y, 0.0)),
                    scale: monster_scale,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Monster)
            .insert(Stats::monster_stats(monster_type))
            .insert(MonsterMovement {
                direction,
                idle_speed,
                aggressive_speed,
            })
            .insert(MovementTimer(Timer::from_seconds(
                timer_duration,
                TimerMode::Repeating,
            )))
            .insert(animation_indices)
            .insert(AnimationTimer(Timer::from_seconds(
                0.5,
                TimerMode::Repeating,
            )))
            .insert(AttackTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert(MonsterState::Idle)
            .with_children(|parent| {
                // Health Bar Background
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(5.0),
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(40.0),
                                ..Default::default()
                            },
                            background_color: BackgroundColor(Color::DARK_GRAY),
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..Default::default()
                        },
                        MonsterHealthBarBackground,
                    ))
                    .with_children(|parent| {
                        // Health Bar Fill
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..Default::default()
                                },
                                background_color: BackgroundColor(Color::GREEN),
                                ..Default::default()
                            },
                            MonsterHealthBar,
                        ));
                    });
            });
    }
}
