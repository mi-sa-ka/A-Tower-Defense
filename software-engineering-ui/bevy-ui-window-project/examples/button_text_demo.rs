use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct ButtonLabel;

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<InputFocus>()
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, fps_text_update_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.1)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: px(220),
                        height: px(72),
                        border: UiRect::all(px(4)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Button"),
                        TextFont {
                            font_size: 34.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ButtonLabel,
                    ));
                });
        });

    commands.spawn((
        Text::new("FPS: --"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            left: px(16),
            ..default()
        },
        FpsText,
    ));
}

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text, With<ButtonLabel>>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, children) in
        &mut interaction_query
    {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match *interaction {
                Interaction::Pressed => {
                    input_focus.set(entity);
                    **text = "Press".to_string();
                    *color = PRESSED_BUTTON.into();
                    *border_color = BorderColor::all(Color::srgb(0.9, 0.2, 0.2));
                    button.set_changed();
                }
                Interaction::Hovered => {
                    input_focus.set(entity);
                    **text = "Hover".to_string();
                    *color = HOVERED_BUTTON.into();
                    *border_color = BorderColor::all(Color::WHITE);
                    button.set_changed();
                }
                Interaction::None => {
                    input_focus.clear();
                    **text = "Button".to_string();
                    *color = NORMAL_BUTTON.into();
                    *border_color = BorderColor::all(Color::BLACK);
                }
            }
        }
    }
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|v| v.smoothed())
        .unwrap_or_default();

    for mut text in &mut query {
        text.0 = format!("FPS: {fps:.2}");
    }
}
