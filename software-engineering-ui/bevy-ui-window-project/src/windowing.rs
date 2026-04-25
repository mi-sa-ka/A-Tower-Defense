use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{CursorGrabMode, PresentMode, PrimaryWindow, WindowLevel, WindowRef};

pub fn spawn_secondary_window(mut commands: Commands) {
    let second_window = commands
        .spawn(Window {
            title: "调试窗口".to_string(),
            resolution: (640.0, 360.0).into(),
            ..default()
        })
        .id();

    commands.spawn((
        Camera2d,
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(second_window)),
            order: 1,
            ..default()
        },
    ));
}

pub fn toggle_vsync_system(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !input.just_pressed(KeyCode::KeyV) {
        return;
    }

    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.present_mode = if matches!(window.present_mode, PresentMode::AutoVsync) {
        PresentMode::AutoNoVsync
    } else {
        PresentMode::AutoVsync
    };

    info!("PRESENT_MODE: {:?}", window.present_mode);
}

pub fn switch_window_level_system(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !input.just_pressed(KeyCode::KeyT) {
        return;
    }

    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.window_level = match window.window_level {
        WindowLevel::AlwaysOnBottom => WindowLevel::Normal,
        WindowLevel::Normal => WindowLevel::AlwaysOnTop,
        WindowLevel::AlwaysOnTop => WindowLevel::AlwaysOnBottom,
    };

    info!("WINDOW_LEVEL: {:?}", window.window_level);
}

pub fn toggle_cursor_grab_system(
    input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !input.just_pressed(KeyCode::KeyC) {
        return;
    }

    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };

    window.cursor_options.grab_mode = match window.cursor_options.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked => CursorGrabMode::Confined,
        CursorGrabMode::Confined => CursorGrabMode::None,
    };

    info!("CURSOR_GRAB_MODE: {:?}", window.cursor_options.grab_mode);
}
