use bevy::prelude::*;
use crate::constants::{TOWER_COST, LASER_TOWER_COST, 
                       TOWER_UPGRADE_BASE_COST, LASER_UPGRADE_BASE_COST, 
                       UPGRADE_DAMAGE_MULTIPLIER, UPGRADE_RANGE_MULTIPLIER};
use crate::components::{Tower, LaserTower};
use crate::resources::{GameState, GameStatus, TowerType, SelectedTower};
use bevy::input::keyboard::KeyCode;

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
    mut ui_query: Query<Entity, With<TowerSelectionUI>>,
) {
    if *game_status != GameStatus::Playing {
        return;
    }

    // 清理旧的 UI
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let font = asset_server.load(r"C:\Windows\Fonts\arial.ttf");

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    bottom: Val::Px(20.0),
                    ..Default::default()
                },
                overflow: Overflow::Visible,   // 修复：Visible 大写
                ..Default::default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
            ..Default::default()
        },
        TowerSelectionUI,
    )).with_children(|parent| {
        // 标题
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

        // 选中指示器
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

        // 普通塔按钮
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

        // 激光塔按钮
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
            if game_state.selected_tower == TowerType::Basic {
                game_state.selected_tower = TowerType::None;
                println!("Basic Tower deselected");
            } else {
                game_state.selected_tower = TowerType::Basic;
                println!("Basic Tower selected");
            }
        }
    }

    for interaction in laser_tower_button_query.iter() {
        if *interaction == Interaction::Clicked {
            if game_state.selected_tower == TowerType::Laser {
                game_state.selected_tower = TowerType::None;
                println!("Laser Tower deselected");
            } else {
                game_state.selected_tower = TowerType::Laser;
                println!("Laser Tower selected");
            }
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


pub fn keyboard_upgrade(
    keyboard_input: Res<Input<KeyCode>>,
    selected_tower: Res<SelectedTower>,
    mut game_state: ResMut<GameState>,
    game_status: Res<GameStatus>,
    mut tower_query: Query<&mut Tower>,
    mut laser_query: Query<&mut LaserTower>,
) {
    println!(">>> keyboard_upgrade called");  // 调试打印

    if *game_status != GameStatus::Playing {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::U) {
        println!(">>> U key pressed");
        if let Some(tower_entity) = selected_tower.0 {
            // 先尝试普通塔
            if let Ok(mut tower) = tower_query.get_mut(tower_entity) {
                let cost = TOWER_UPGRADE_BASE_COST * tower.level;
                if game_state.money >= cost {
                    game_state.money -= cost;
                    tower.level += 1;
                    tower.damage *= 1.0 + UPGRADE_DAMAGE_MULTIPLIER;
                    tower.range *= 1.0 + UPGRADE_RANGE_MULTIPLIER;
                    println!("[Upgrade] Tower upgraded to level {}! Damage: {:.1}, Range: {:.1}", 
                             tower.level, tower.damage, tower.range);
                } else {
                    println!("[Upgrade] Not enough money! Need ${}", cost);
                }
            } 
            // 否则尝试激光塔
            else if let Ok(mut laser) = laser_query.get_mut(tower_entity) {
                let cost = LASER_UPGRADE_BASE_COST * laser.level;
                if game_state.money >= cost {
                    game_state.money -= cost;
                    laser.level += 1;
                    laser.damage *= 1.0 + UPGRADE_DAMAGE_MULTIPLIER;
                    laser.range *= 1.0 + UPGRADE_RANGE_MULTIPLIER;
                    println!("[Upgrade] Laser upgraded to level {}! Damage: {:.1}, Range: {:.1}", 
                             laser.level, laser.damage, laser.range);
                } else {
                    println!("[Upgrade] Not enough money! Need ${}", cost);
                }
            } else {
                println!("[Upgrade] Selected entity is not a tower");
            }
        } else {
            println!("[Upgrade] No tower selected");
        }
    }
}