use bevy::prelude::*;
use crate::components::{Bullet, Enemy, Tower};
use crate::constants::BULLET_SPEED;
use crate::resources::GameStatus;

pub fn tower_targeting(
    mut commands: Commands,
    mut tower_query: Query<(&Transform, &mut Tower)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Tower>)>,
    time: Res<Time>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for (tower_transform, mut tower) in tower_query.iter_mut() {
        tower.attack_cooldown.tick(time.delta());

        if tower.attack_cooldown.finished() {
            let mut closest_enemy: Option<(&Transform, f32)> = None;

            for enemy_transform in enemy_query.iter() {
                let distance = tower_transform
                    .translation
                    .distance(enemy_transform.translation);

                if distance <= tower.range {
                    if closest_enemy.is_none() || distance < closest_enemy.unwrap().1 {
                        closest_enemy = Some((enemy_transform, distance));
                    }
                }
            }

            if let Some((enemy_transform, _)) = closest_enemy {
                let direction = (enemy_transform.translation - tower_transform.translation)
                    .truncate()
                    .normalize_or_zero();

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(tower_transform.translation),
                    ..Default::default()
                }).insert(Bullet {
                    velocity: direction * BULLET_SPEED,
                    damage: tower.damage,
                });

                tower.attack_cooldown.reset();
            }
        }
    }
}