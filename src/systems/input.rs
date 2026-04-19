use bevy::prelude::*;
use crate::components::{BuildTile, LaserTower, Tower};
use crate::constants::{TILE_SIZE, TOWER_ATTACK_COOLDOWN, TOWER_COST, TOWER_DAMAGE, TOWER_RANGE, LASER_TOWER_ATTACK_COOLDOWN, LASER_TOWER_COST, LASER_TOWER_DAMAGE, LASER_TOWER_RANGE};
use crate::resources::{GameState, GameStatus, MousePosition, TowerType};

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
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
    asset_server: Res<AssetServer>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        match game_state.selected_tower {
            TowerType::Laser if game_state.money >= LASER_TOWER_COST => {
                if let Some(world_pos) = mouse_pos.position {
                    for build_tile_transform in build_tile_query.iter() {
                        let tile_pos = build_tile_transform.translation.truncate();

                        if world_pos.distance(tile_pos) < TILE_SIZE / 2.0 {
                            commands.spawn((
                                SpriteBundle {
                                    texture: asset_server.load("laser_tower.png"),
                                    transform: Transform::from_translation(
                                        build_tile_transform.translation + Vec3::new(0.0, 0.0, 1.0)
                                    )
                                    .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                                    ..Default::default()
                                },
                                LaserTower {
                                    range: LASER_TOWER_RANGE,
                                    damage: LASER_TOWER_DAMAGE,
                                    attack_cooldown: Timer::from_seconds(LASER_TOWER_ATTACK_COOLDOWN, TimerMode::Repeating),
                                },
                            ));

                            game_state.money -= LASER_TOWER_COST;
                            break;
                        }
                    }
                }
            }
            TowerType::Basic if game_state.money >= TOWER_COST => {
                if let Some(world_pos) = mouse_pos.position {
                    for build_tile_transform in build_tile_query.iter() {
                        let tile_pos = build_tile_transform.translation.truncate();

                        if world_pos.distance(tile_pos) < TILE_SIZE / 2.0 {
                            commands.spawn((
                                SpriteBundle {
                                    texture: asset_server.load("normal_tower.png"),
                                    transform: Transform::from_translation(
                                        build_tile_transform.translation + Vec3::new(0.0, 0.0, 1.0)
                                    )
                                    .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                                    ..Default::default()
                                },
                                Tower {
                                    range: TOWER_RANGE,
                                    damage: TOWER_DAMAGE,
                                    attack_cooldown: Timer::from_seconds(TOWER_ATTACK_COOLDOWN, TimerMode::Repeating),
                                },
                            ));

                            game_state.money -= TOWER_COST;
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}