use bevy::prelude::*;
use crate::components::{Bullet, Enemy, HealthBar, ShieldedEnemy};
use crate::resources::{GameState, GameStatus};

pub fn bullet_movement(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet)>,
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform, &mut Sprite), Without<Bullet>>,
    mut shielded_query: Query<&mut ShieldedEnemy>,
    health_bar_query: Query<(Entity, &HealthBar)>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for (bullet_entity, mut transform, bullet) in bullet_query.iter_mut() {
        transform.translation += (bullet.velocity * time.delta_seconds()).extend(0.0);

        for (enemy_entity, mut enemy, enemy_transform, mut sprite) in enemy_query.iter_mut() {
            if transform.translation.distance(enemy_transform.translation) < 20.0 {
                if let Ok(mut shielded) = shielded_query.get_mut(enemy_entity) {
                    if shielded.has_shield {
                        shielded.has_shield = false;
                        sprite.color = Color::BLUE;
                    } else {
                        enemy.health -= bullet.damage;

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

                            println!("Enemy killed by bullet! Money: {}, Enemies Killed: {}, Enemy Count: {}", game_state.money, game_state.enemies_killed, game_state.enemy_count);
                            commands.entity(enemy_entity).despawn();
                        }
                    }
                } else {
                    enemy.health -= bullet.damage;

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

                        println!("Enemy killed by bullet! Money: {}, Enemies Killed: {}, Enemy Count: {}", game_state.money, game_state.enemies_killed, game_state.enemy_count);
                        commands.entity(enemy_entity).despawn();
                    }
                }
                commands.entity(bullet_entity).despawn();
                break;
            }
        }

        if transform.translation.length() > 1000.0 {
            commands.entity(bullet_entity).despawn();
        }
    }
}