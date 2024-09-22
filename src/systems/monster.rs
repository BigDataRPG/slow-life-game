// File: src/systems/monster.rs
use crate::components::{
    monster::{Monster, MonsterHealthBar, MonsterHealthBarBackground},
    stats::Stats,
};
use crate::resources::game_assets::GameAssets;
use crate::utils::{calculate_scale, snap_to_grid};
use crate::MonsterRespawnTimer;
use bevy::prelude::*;
use rand::prelude::*;

pub fn monster_respawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MonsterRespawnTimer>,
    assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
    query: Query<(), With<Monster>>, // To count existing monsters
) {
    timer.timer.tick(time.delta());

    if timer.timer.finished() && query.iter().count() < 5 {
        // Define the spawn area boundaries
        let spawn_area = -250.0..250.0; // Removed unnecessary parentheses

        // Generate random positions
        let mut rng = thread_rng();
        let x = rng.gen_range(spawn_area.clone());
        let y = rng.gen_range(spawn_area.clone());

        // Calculate monster scale
        let monster_scale = calculate_scale(&assets.monster, &images);

        // Spawn a monster
        commands
            .spawn(SpriteBundle {
                texture: assets.monster.clone(),
                transform: Transform {
                    translation: snap_to_grid(Vec3::new(x, y, 0.0)),
                    scale: monster_scale,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Monster)
            .insert(Stats::new(50, 8, 3))
            .with_children(|parent| {
                // Health Bar Background
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(32.0),
                                height: Val::Px(5.0),
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(40.0), // Position above the monster
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
