use bevy::prelude::*;
use bevy::ecs::query::Or;
use crate::components::{BuildTile, MoneyDisplay, PathTile, WaveDisplay};
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use crate::resources::GameStatus;

pub fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_status: Res<GameStatus>,
    camera_query: Query<Entity, With<Camera>>,
    money_display_query: Query<Entity, With<MoneyDisplay>>,
    map_query: Query<Entity, Or<(With<PathTile>, With<BuildTile>)>>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    if !camera_query.is_empty() && !money_display_query.is_empty() && !map_query.is_empty() {
        return;
    }

    if camera_query.is_empty() {
        commands.spawn(Camera2dBundle::default());
    }

    if money_display_query.is_empty() {
        let font = asset_server.load(r"C:\Windows\Fonts\arial.ttf");

        commands.spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Money: $100".to_string(),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 32.0,
                                color: Color::GOLD,
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            MoneyDisplay,
        ));

        commands.spawn((
            TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Wave: 1".to_string(),
                            style: TextStyle {
                                font,
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(20.0),
                        top: Val::Px(60.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            WaveDisplay,
        ));
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let position = Vec3::new(
                x as f32 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                y as f32 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                0.0,
            );

            let is_path = (x == 0 && y == 6) || (x <= 5 && y == 6) ||
                         (x == 5 && y >= 6 && y <= 9) || (x >= 5 && x <= 10 && y == 9) ||
                         (x == 10 && y >= 3 && y <= 9) || (x >= 10 && x <= 14 && y == 3);

            let color = if is_path { Color::GRAY } else { Color::DARK_GREEN };

            if is_path {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    },
                    PathTile,
                ));
            } else {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(position),
                        ..Default::default()
                    },
                    BuildTile,
                ));
            }
        }
    }
}