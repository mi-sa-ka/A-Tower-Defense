use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(14),
                padding: UiRect::all(px(14)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.07, 0.09)),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Flex + Grid Layout"),
                TextFont {
                    font: font.clone(),
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.95, 0.98)),
            ));

            root
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(140),
                        flex_direction: FlexDirection::Row,
                        column_gap: px(12),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.11, 0.12, 0.16)),
                    BorderRadius::all(px(10)),
                ))
                .with_children(|flex| {
                    for i in 1..=3 {
                        flex.spawn((
                            Node {
                                flex_grow: 1.0,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2 + i as f32 * 0.08, 0.26, 0.34)),
                            BorderRadius::all(px(8)),
                            children![(
                                Text::new(format!("Flex {i}")),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            )],
                        ));
                    }
                });

            root
                .spawn((
                    Node {
                        width: percent(100),
                        flex_grow: 1.0,
                        display: Display::Grid,
                        grid_template_columns: RepeatedGridTrack::flex(3, 1.0),
                        grid_template_rows: RepeatedGridTrack::px(2, 110.0),
                        column_gap: px(12),
                        row_gap: px(12),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.09, 0.1, 0.14)),
                    BorderRadius::all(px(10)),
                    Padding::all(px(12)),
                ))
                .with_children(|grid| {
                    for i in 1..=6 {
                        let mut node = Node {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        };

                        if i == 1 {
                            node.grid_column = GridPlacement::span(2);
                        }

                        grid.spawn((
                            node,
                            BackgroundColor(Color::srgb(0.15, 0.18 + i as f32 * 0.05, 0.28)),
                            BorderRadius::all(px(8)),
                            children![(
                                Text::new(format!("Grid {i}")),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            )],
                        ));
                    }
                });
        });
}
