use bevy::prelude::*;
use crate::components::{Enemy, HealthBar, ShieldedEnemy};
use crate::constants::{ENEMIES_PER_WAVE, ENEMY_BOUNTY, ENEMY_HEALTH, ENEMY_SPEED, HEALTH_BAR_HEIGHT, HEALTH_BAR_WIDTH};
use crate::resources::{EnemyPath, GameState, GameStatus, SpawnTimer};
use rand::Rng;

pub fn enemy_movement(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Transform, &mut Enemy), Without<HealthBar>>,
    path: Res<EnemyPath>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    health_bar_query: Query<(Entity, &Transform, &HealthBar)>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    let mut despawn_list: Vec<Entity> = Vec::new();

    for (entity, mut transform, mut enemy) in enemy_query.iter_mut() {
        if enemy.path_index >= path.points.len() {
            despawn_list.push(entity);
            continue;
        }

        let target = path.points[enemy.path_index];
        let direction = (target - transform.translation.truncate()).normalize_or_zero();

        transform.translation += (direction * enemy.speed * time.delta_seconds()).extend(0.0);

        if transform.translation.truncate().distance(target) < 5.0 {
            enemy.path_index += 1;
        }
    }

    for entity in despawn_list {
        for (hb_entity, _, health_bar) in health_bar_query.iter() {
            if health_bar.enemy_entity == entity {
                commands.entity(hb_entity).despawn();
            }
        }
        game_state.lives = game_state.lives.saturating_sub(1);
        game_state.enemy_count = game_state.enemy_count.saturating_sub(1);
        game_state.enemies_escaped += 1;
        commands.entity(entity).despawn();
        println!("Enemy escaped! Lives: {}, Enemies Escaped: {}, Enemy Count: {}", game_state.lives, game_state.enemies_escaped, game_state.enemy_count);
    }

    let enemies_per_wave = ENEMIES_PER_WAVE;
    let total_enemies_processed = game_state.enemies_killed + game_state.enemies_escaped;
    
    println!("DEBUG: Wave: {}, Enemies Killed: {}, Enemies Escaped: {}, Total Processed: {}, Enemies Per Wave: {}, Has Spawned: {}", 
             game_state.wave, game_state.enemies_killed, game_state.enemies_escaped, 
             total_enemies_processed, enemies_per_wave, game_state.has_spawned_enemies);
    
    if game_state.has_spawned_enemies && total_enemies_processed >= enemies_per_wave {
        game_state.wave += 1;
        game_state.enemies_killed = 0;
        game_state.enemies_escaped = 0;
        game_state.has_spawned_enemies = false;
        println!("Wave {} started!", game_state.wave);
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    path: Res<EnemyPath>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    spawn_timer.timer.tick(time.delta());

    let enemies_per_wave = ENEMIES_PER_WAVE;
    let total_enemies_spawned = game_state.enemies_killed + game_state.enemies_escaped + game_state.enemy_count;
    
    if total_enemies_spawned < enemies_per_wave && spawn_timer.timer.finished() {
        let enemy_pos = path.points[0].extend(2.0);

        let mut rng = rand::thread_rng();
        let is_shielded = rng.gen_bool(0.3);
        
        let color = if is_shielded {
            Color::PURPLE
        } else {
            Color::BLUE
        };
        
        let enemy_entity = if is_shielded {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(enemy_pos),
                ..Default::default()
            }).insert(Enemy {
                speed: ENEMY_SPEED,
                health: ENEMY_HEALTH * game_state.wave as f32,
                max_health: ENEMY_HEALTH * game_state.wave as f32,
                path_index: 1,
                bounty: ENEMY_BOUNTY * game_state.wave,
            }).insert(ShieldedEnemy {
                has_shield: true,
            }).id()
        } else {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(enemy_pos),
                ..Default::default()
            }).insert(Enemy {
                speed: ENEMY_SPEED,
                health: ENEMY_HEALTH * game_state.wave as f32,
                max_health: ENEMY_HEALTH * game_state.wave as f32,
                path_index: 1,
                bounty: ENEMY_BOUNTY * game_state.wave,
            }).id()
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform::from_translation(enemy_pos + Vec3::new(0.0, 25.0, 3.0)),
                ..Default::default()
            },
            HealthBar {
                width: HEALTH_BAR_WIDTH,
                height: HEALTH_BAR_HEIGHT,
                enemy_entity,
            },
        ));

        game_state.enemy_count += 1;
        game_state.has_spawned_enemies = true;
        spawn_timer.timer.reset();
        println!("Spawned enemy {} of wave {}", game_state.enemy_count, game_state.wave);
    }
}

pub fn update_health_bars(
    enemy_query: Query<(Entity, &Transform, &Enemy), Without<HealthBar>>,
    mut health_bar_query: Query<(&mut Transform, &mut Sprite, &HealthBar)>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for (mut health_bar_transform, mut sprite, health_bar) in health_bar_query.iter_mut() {
        if let Ok((_, enemy_transform, enemy)) = enemy_query.get(health_bar.enemy_entity) {
            health_bar_transform.translation.x = enemy_transform.translation.x;
            health_bar_transform.translation.y = enemy_transform.translation.y + 25.0;
            health_bar_transform.translation.z = 3.0;

            let health_ratio = (enemy.health / enemy.max_health).max(0.0);
            sprite.custom_size = Some(Vec2::new(health_bar.width * health_ratio, health_bar.height));

            if health_ratio > 0.6 {
                sprite.color = Color::rgb(0.0, 1.0, 0.0);
            } else if health_ratio > 0.3 {
                sprite.color = Color::rgb(1.0, 1.0, 0.0);
            } else {
                sprite.color = Color::rgb(1.0, 0.0, 0.0);
            }
        }
    }
}