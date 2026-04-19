use bevy::prelude::*;
use crate::components::{Tower, LaserTower};
use crate::constants::{LASER_TOWER_COST, TOWER_COST};
use crate::resources::{GameStatus, GameState, TowerType};

#[derive(Component)]
pub struct TowerSelectionUI;

#[derive(Component)]
pub struct TowerButton;

#[derive(Component)]
pub struct LaserTowerButton;

#[derive(Component)]
pub struct TowerName;

#[derive(Component)]
pub struct SelectedTowerIndicator;

pub fn setup_tower_selection(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_status: Res<GameStatus>,
    ui_query: Query<Entity, With<TowerSelectionUI>>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    if !ui_query.is_empty() {
        return;
    }

    let font = asset_server.load(r"C:\Windows\Fonts\arial.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
            ..Default::default()
        },
        TowerSelectionUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Towers",
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..Default::default()
        }));

        parent.spawn((
            TextBundle::from_section(
                "Selected: None",
                TextStyle {
                    font: font.clone(),
                    font_size: 14.0,
                    color: Color::YELLOW,
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..Default::default()
            }),
            SelectedTowerIndicator,
        ));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..Default::default()
                },
                background_color: Color::YELLOW.into(),
                ..Default::default()
            },
            TowerButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("${}", TOWER_COST),
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::BLACK,
                },
            ));
        });

        parent.spawn(TextBundle::from_section(
            "Basic Tower",
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..Default::default()
        }));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(80.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..Default::default()
                },
                background_color: Color::CYAN.into(),
                ..Default::default()
            },
            LaserTowerButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("${}", LASER_TOWER_COST),
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::BLACK,
                },
            ));
        });

        parent.spawn(TextBundle::from_section(
            "Laser Tower",
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..Default::default()
        }));
    });
}

pub fn handle_tower_selection(
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
    tower_button_query: Query<&Interaction, (With<TowerButton>, Changed<Interaction>)>,
    laser_tower_button_query: Query<&Interaction, (With<LaserTowerButton>, Changed<Interaction>)>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for interaction in tower_button_query.iter() {
        if *interaction == Interaction::Clicked {
            game_state.selected_tower = TowerType::Basic;
            println!("Basic Tower selected");
        }
    }

    for interaction in laser_tower_button_query.iter() {
        if *interaction == Interaction::Clicked {
            game_state.selected_tower = TowerType::Laser;
            println!("Laser Tower selected");
        }
    }
}

pub fn update_selected_tower_indicator(
    game_state: Res<GameState>,
    mut indicator_query: Query<&mut Text, With<SelectedTowerIndicator>>,
    game_status: Res<GameStatus>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    for mut text in indicator_query.iter_mut() {
        match game_state.selected_tower {
            TowerType::None => {
                text.sections[0].value = "Selected: None".to_string();
            }
            TowerType::Basic => {
                text.sections[0].value = "Selected: Basic Tower".to_string();
            }
            TowerType::Laser => {
                text.sections[0].value = "Selected: Laser Tower".to_string();
            }
        }
    }
}