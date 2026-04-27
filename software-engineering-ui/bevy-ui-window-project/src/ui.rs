use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::gameplay::{BuildResultEvent, BuildTowerEvent, GameStats};

const BUTTON_NORMAL: Color = Color::srgb(0.16, 0.17, 0.2);
const BUTTON_HOVERED: Color = Color::srgb(0.24, 0.29, 0.36);
const BUTTON_PRESSED: Color = Color::srgb(0.31, 0.5, 0.34);

#[derive(Component)]
pub struct BuildTowerButton;

#[derive(Component)]
pub struct BuildButtonLabel;

#[derive(Component)]
pub struct BuildStatusText;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct WaveText;

#[derive(Component)]
pub struct FpsText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let zh_font = asset_server.load("fonts/msyh.ttc");

    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(72.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.07, 0.1, 0.92)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("金币: 0"),
                TextFont {
                    font: zh_font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.98, 0.84, 0.2)),
                GoldText,
            ));

            parent.spawn((
                Text::new("生命值: 0"),
                TextFont {
                    font: zh_font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.93, 0.35, 0.35)),
                HealthText,
            ));

            parent.spawn((
                Text::new("波次: 0"),
                TextFont {
                    font: zh_font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.86, 1.0)),
                WaveText,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(24.0),
                top: Val::Percent(42.0),
                width: Val::Px(240.0),
                height: Val::Px(130.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    BuildTowerButton,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::srgb(0.9, 0.9, 0.92)),
                    BorderRadius::all(Val::Px(12.0)),
                    BackgroundColor(BUTTON_NORMAL),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("建造塔楼 (50金币)"),
                        TextFont {
                            font: zh_font.clone(),
                            font_size: 26.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        BuildButtonLabel,
                    ));
                });

            parent.spawn((
                Text::new("Ready"),
                TextFont {
                    font: zh_font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.78, 0.88, 1.0)),
                BuildStatusText,
            ));
        });

    commands.spawn((
        Text::new("FPS: --"),
        TextFont {
            font: zh_font,
            font_size: 22.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 0.95)),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(16.0),
            bottom: Val::Px(14.0),
            ..default()
        },
        FpsText,
    ));
}

pub fn build_status_feedback_system(
    mut result_reader: EventReader<BuildResultEvent>,
    mut status_query: Query<&mut Text, With<BuildStatusText>>,
) {
    for result in result_reader.read() {
        for mut status in &mut status_query {
            status.0 = if result.success {
                "Construction complete".to_string()
            } else {
                "Not enough gold".to_string()
            };
        }
    }
}

pub fn build_button_interaction_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<BuildTowerButton>),
    >,
    mut text_query: Query<&mut Text, With<BuildButtonLabel>>,
    mut build_writer: EventWriter<BuildTowerEvent>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Pressed => {
                    **text = "建造中...".to_string();
                    *color = BUTTON_PRESSED.into();
                    *border_color = BorderColor(Color::srgb(0.8, 1.0, 0.8));
                    build_writer.send(BuildTowerEvent);
                }
                Interaction::Hovered => {
                    **text = "建造塔楼 (50金币)".to_string();
                    *color = BUTTON_HOVERED.into();
                    *border_color = BorderColor(Color::WHITE);
                }
                Interaction::None => {
                    **text = "建造塔楼 (50金币)".to_string();
                    *color = BUTTON_NORMAL.into();
                    *border_color = BorderColor(Color::srgb(0.9, 0.9, 0.92));
                }
            }
        }
    }
}

pub fn hud_text_update_system(
    stats: Res<GameStats>,
    mut hud_query: Query<
        (
            &mut Text,
            Option<&GoldText>,
            Option<&HealthText>,
            Option<&WaveText>,
        ),
    >,
) {
    for (mut text, is_gold, is_health, is_wave) in &mut hud_query {
        if is_gold.is_some() {
            text.0 = format!("金币: {}", stats.gold);
        }
        if is_health.is_some() {
            text.0 = format!("生命值: {}", stats.health);
        }
        if is_wave.is_some() {
            text.0 = format!("波次: {}", stats.wave);
        }
    }
}

pub fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or_default();

    for mut text in &mut query {
        text.0 = format!("FPS: {fps:.2}");
    }
}
