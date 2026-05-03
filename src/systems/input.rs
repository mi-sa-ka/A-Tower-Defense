use bevy::prelude::*;
use crate::components::{BuildTile, LaserTower, Tower};
use crate::constants::{TILE_SIZE, TOWER_ATTACK_COOLDOWN, TOWER_COST, TOWER_DAMAGE, TOWER_RANGE, LASER_TOWER_ATTACK_COOLDOWN, LASER_TOWER_COST, LASER_TOWER_DAMAGE, LASER_TOWER_RANGE, TOWER_SELL_RETURN_RATIO};
use crate::resources::{GameState, GameStatus, MousePosition, TowerType, SelectedTower};
use crate::components::TowerEntity; 
use bevy::input::keyboard::KeyCode;

pub fn update_mouse_position(
    mut mouse_pos: ResMut<MousePosition>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_events: EventReader<CursorMoved>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    if let Some(cursor_event) = cursor_events.iter().last() {
        for (camera, camera_transform) in camera_query.iter() {
            if camera.order == 0 {
                if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_event.position) {
                    mouse_pos.position = Some(world_pos);
                }
                break;
            }
        }
    }
}

pub fn handle_build_input(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePosition>,
    build_tile_query: Query<&Transform, (With<BuildTile>, Without<Tower>, Without<LaserTower>)>,
    existing_tower_query: Query<(&Transform, Entity), With<TowerEntity>>,
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
    asset_server: Res<AssetServer>,
    mut selected_tower: ResMut<SelectedTower>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = mouse_pos.position {
            // 1. 检测是否点击到塔
            let mut clicked_tower = None;
            for (transform, entity) in existing_tower_query.iter() {
                let tile_pos = transform.translation.truncate();
                if world_pos.distance(tile_pos) < TILE_SIZE / 2.0 {
                    clicked_tower = Some(entity);
                    break;
                }
            }

            if let Some(tower_entity) = clicked_tower {
                // 点击到塔：选中它，并清除指示器（实际上选中后指示器会显示）
                selected_tower.0 = Some(tower_entity);
                game_state.selected_tower = TowerType::None; 
                println!("Tower selected");
                return;
            }

            // 2. 没有点击到塔 → 先清除当前选中的塔（范围指示器消失）
            selected_tower.0 = None;

            // 3. 尝试建造
            match game_state.selected_tower {
                TowerType::Laser if game_state.money >= LASER_TOWER_COST => {
                    for build_tile_transform in build_tile_query.iter() {
                        let tile_pos = build_tile_transform.translation.truncate();
                        if world_pos.distance(tile_pos) < TILE_SIZE / 2.0 {
                            // 检查该位置是否已有塔
                            let mut occupied = false;
                            for (transform, _) in existing_tower_query.iter() {
                                if transform.translation.truncate().distance(tile_pos) < 10.0 {
                                    occupied = true;
                                    break;
                                }
                            }
                            if !occupied {
                                let new_tower = commands.spawn((
                                    SpriteBundle {
                                        texture: asset_server.load("laser_tower.png"),
                                        transform: Transform::from_translation(
                                            build_tile_transform.translation + Vec3::new(0.0, 0.0, 1.0)
                                        ),
                                        ..Default::default()
                                    },
                                    LaserTower {
                                        range: LASER_TOWER_RANGE,
                                        damage: LASER_TOWER_DAMAGE,
                                        attack_cooldown: Timer::from_seconds(LASER_TOWER_ATTACK_COOLDOWN, TimerMode::Repeating),
                                        level: 1, 
                                    },
                                    TowerEntity,
                                )).id();
                                game_state.money -= LASER_TOWER_COST;
                                // 建造后自动选中新塔
                                selected_tower.0 = Some(new_tower);
                                println!("Laser tower built and selected!");
                            }
                            break;
                        }
                    }
                }
                TowerType::Basic if game_state.money >= TOWER_COST => {
                    for build_tile_transform in build_tile_query.iter() {
                        let tile_pos = build_tile_transform.translation.truncate();
                        if world_pos.distance(tile_pos) < TILE_SIZE / 2.0 {
                            let mut occupied = false;
                            for (transform, _) in existing_tower_query.iter() {
                                if transform.translation.truncate().distance(tile_pos) < 10.0 {
                                    occupied = true;
                                    break;
                                }
                            }
                            if !occupied {
                                let new_tower = commands.spawn((
                                    SpriteBundle {
                                        texture: asset_server.load("normal_tower.png"),
                                        transform: Transform::from_translation(
                                            build_tile_transform.translation + Vec3::new(0.0, 0.0, 1.0)
                                        ),
                                        ..Default::default()
                                    },
                                    Tower {
                                        range: TOWER_RANGE,
                                        damage: TOWER_DAMAGE,
                                        attack_cooldown: Timer::from_seconds(TOWER_ATTACK_COOLDOWN, TimerMode::Repeating),
                                        level: 1, 
                                    },
                                    TowerEntity,
                                )).id();
                                game_state.money -= TOWER_COST;
                                selected_tower.0 = Some(new_tower);
                                println!("Basic tower built and selected!");
                            }
                            break;
                        }
                    }
                }
                _ => {
                    // 未选择塔或金钱不足：已经清除了选中，无需额外操作
                }
            }
        }
    }
}

pub fn handle_sell_tower(
    mut commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MousePosition>,
    tower_query: Query<(&Transform, Entity, Option<&Tower>, Option<&LaserTower>), With<TowerEntity>>,
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
    mut selected_tower: ResMut<SelectedTower>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        if let Some(world_pos) = mouse_pos.position {
            for (transform, entity, maybe_tower, maybe_laser) in tower_query.iter() {
                let tile_pos = transform.translation.truncate();
                if world_pos.distance(tile_pos) < TILE_SIZE / 2.0 {
                    let cost = if maybe_tower.is_some() {
                        TOWER_COST
                    } else if maybe_laser.is_some() {
                        LASER_TOWER_COST
                    } else {
                        0
                    };
                    let refund = (cost as f32 * TOWER_SELL_RETURN_RATIO) as u32;
                    game_state.money += refund;
                    if selected_tower.0 == Some(entity) {
                        selected_tower.0 = None;
                    }
                    commands.entity(entity).despawn();
                    println!("Tower sold! Refund: {}", refund);
                    break;
                }
            }
        }
    }
}

pub fn cancel_build_mode(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.selected_tower = TowerType::None;
        println!("Build mode cancelled (ESC)");
    }
}