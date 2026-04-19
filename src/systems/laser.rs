use bevy::prelude::*;
use crate::components::{Enemy, HealthBar, Laser, LaserTower, ShieldedEnemy};
use crate::resources::{GameState, GameStatus};

pub fn laser_tower_targeting(
    mut commands: Commands,
    mut tower_query: Query<(&Transform, &mut LaserTower)>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<LaserTower>)>,
    time: Res<Time>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for (tower_transform, mut tower) in tower_query.iter_mut() {
        tower.attack_cooldown.tick(time.delta());

        if tower.attack_cooldown.finished() {
            let mut closest_enemy: Option<(Entity, &Transform, f32)> = None;

            for (enemy_entity, enemy_transform) in enemy_query.iter() {
                let distance = tower_transform
                    .translation
                    .distance(enemy_transform.translation);

                if distance <= tower.range {
                    if closest_enemy.is_none() || distance < closest_enemy.unwrap().2 {
                        closest_enemy = Some((enemy_entity, enemy_transform, distance));
                    }
                }
            }

            if let Some((enemy_entity, enemy_transform, _)) = closest_enemy {
                let infinite_range = 10000.0;

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::CYAN,
                        custom_size: Some(Vec2::new(5.0, infinite_range)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: tower_transform.translation,
                        rotation: Quat::from_rotation_z(
                            (enemy_transform.translation.y - tower_transform.translation.y)
                                .atan2(enemy_transform.translation.x - tower_transform.translation.x) - std::f32::consts::PI / 2.0
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                }).insert(Laser {
                    target: enemy_entity,
                    damage: tower.damage,
                    length: infinite_range,
                });

                tower.attack_cooldown.reset();
            }
        }
    }
}

pub fn laser_damage(
    mut commands: Commands,
    laser_query: Query<(Entity, &Laser, &Transform)>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform)>,
    shielded_query: Query<&ShieldedEnemy>,
    health_bar_query: Query<(Entity, &HealthBar)>,
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for (laser_entity, laser, laser_transform) in laser_query.iter() {
        let laser_direction = laser_transform.rotation * Vec3::Y;
        let laser_start = laser_transform.translation;

        for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation;
            let to_enemy = enemy_pos - laser_start;

            let dot_product = to_enemy.dot(laser_direction);
            if dot_product > 0.0 {
                let distance = line_point_distance(laser_start, laser_start + laser_direction * laser.length, enemy_pos);
                if distance < 20.0 {
                    if let Ok(shielded) = shielded_query.get(enemy_entity) {
                        if !shielded.has_shield {
                            enemy.health -= laser.damage;

                            if enemy.health <= 0.0 {
                                game_state.money += enemy.bounty;
                                game_state.enemies_killed += 1;
                                game_state.enemy_count = game_state.enemy_count.saturating_sub(1);

                                // 销毁血条
                                for (hb_entity, health_bar) in health_bar_query.iter() {
                                    if health_bar.enemy_entity == enemy_entity {
                                        commands.entity(hb_entity).despawn();
                                    }
                                }

                                println!("Enemy killed by laser! Money: {}, Enemies Killed: {}, Enemy Count: {}", game_state.money, game_state.enemies_killed, game_state.enemy_count);
                                commands.entity(enemy_entity).despawn();
                            }
                        }
                    } else {
                        enemy.health -= laser.damage;

                        if enemy.health <= 0.0 {
                            game_state.money += enemy.bounty;
                            game_state.enemies_killed += 1;
                            game_state.enemy_count = game_state.enemy_count.saturating_sub(1);

                            // 销毁血条
                            for (hb_entity, health_bar) in health_bar_query.iter() {
                                if health_bar.enemy_entity == enemy_entity {
                                    commands.entity(hb_entity).despawn();
                                }
                            }

                            println!("Enemy killed by laser! Money: {}, Enemies Killed: {}, Enemy Count: {}", game_state.money, game_state.enemies_killed, game_state.enemy_count);
                            commands.entity(enemy_entity).despawn();
                        }
                    }
                }
            }
        }
        commands.entity(laser_entity).despawn();
    }
}

fn line_point_distance(line_start: Vec3, line_end: Vec3, point: Vec3) -> f32 {
    let line_vec = line_end - line_start;
    let point_vec = point - line_start;
    let line_len = line_vec.length();

    if line_len < 0.001 {
        return point_vec.length();
    }

    let t = point_vec.dot(line_vec) / (line_len * line_len);
    let t_clamped = t.clamp(0.0, 1.0);
    let closest_point = line_start + line_vec * t_clamped;

    (point - closest_point).length()
}