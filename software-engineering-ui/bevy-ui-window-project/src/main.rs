mod audio_control;
mod gameplay;
mod ui;
mod windowing;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowPlugin, WindowTheme};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 软件工程项目 - UI 与窗口管理".into(),
                name: Some("bevy.ui.window.project".into()),
                resolution: (1280.0, 720.0).into(),
                present_mode: PresentMode::AutoVsync,
                window_theme: Some(WindowTheme::Dark),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_event::<gameplay::BuildTowerEvent>()
        .add_event::<gameplay::BuildResultEvent>()
        // Audio module kept but disabled by default.
        // .add_event::<gameplay::TowerShootEvent>()
        // .add_event::<gameplay::EnemyDeathEvent>()
        .add_systems(
            Startup,
            (
                gameplay::setup_gameplay,
                // audio_control::setup_audio,
                windowing::spawn_secondary_window,
                ui::setup_ui,
                // audio_control::start_bgm,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                gameplay::simulate_world_tick,
                gameplay::handle_build_tower_event,
                // gameplay::simulate_tower_combat,
                ui::build_button_interaction_system,
                ui::build_status_feedback_system,
                ui::hud_text_update_system,
                ui::fps_text_update_system,
                // audio_control::play_shoot_sfx,
                // audio_control::play_explode_sfx,
                // audio_control::toggle_audio_system,
                windowing::toggle_vsync_system,
                windowing::switch_window_level_system,
                windowing::toggle_cursor_grab_system,
            ),
        )
        .run();
}
