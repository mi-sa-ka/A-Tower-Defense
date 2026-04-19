use bevy::prelude::*;
use crate::resources::GameStatus;

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_status: Res<GameStatus>,
    menu_ui_query: Query<Entity, With<MenuUI>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    if *game_status != GameStatus::Menu {
        return;
    }

    if camera_query.is_empty() {
        commands.spawn(Camera2dBundle::default());
    }

    if !menu_ui_query.is_empty() {
        return;
    }

    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..Default::default()
        },
        MenuUI,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Tower Defense",
            TextStyle {
                font: asset_server.load("C:\\Windows\\Fonts\\arial.ttf"),
                font_size: 64.0,
                color: Color::GOLD,
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(50.0)),
            ..Default::default()
        }));

        parent.spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(60.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::rgb(0.2, 0.6, 0.2).into(),
                ..Default::default()
            },
            StartButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Start Game",
                TextStyle {
                    font: asset_server.load("C:\\Windows\\Fonts\\arial.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ));
        });

        parent.spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(60.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(20.0)),
                    ..Default::default()
                },
                background_color: Color::rgb(0.6, 0.2, 0.2).into(),
                ..Default::default()
            },
            QuitButton,
        )).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit",
                TextStyle {
                    font: asset_server.load("C:\\Windows\\Fonts\\arial.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

pub fn hide_menu(
    mut commands: Commands,
    game_status: Res<GameStatus>,
    menu_ui_query: Query<Entity, With<MenuUI>>,
) {
    if *game_status == GameStatus::Menu {
        return;
    }

    for entity in menu_ui_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_menu_buttons(
    start_button_query: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    quit_button_query: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut game_status: ResMut<GameStatus>,
) {
    if *game_status != GameStatus::Menu {
        return;
    }

    for interaction in start_button_query.iter() {
        if *interaction == Interaction::Clicked {
            *game_status = GameStatus::Playing;
        }
    }

    for interaction in quit_button_query.iter() {
        if *interaction == Interaction::Clicked {
            std::process::exit(0);
        }
    }
}