pub mod bullet;
pub mod enemy;
pub mod input;
pub mod laser;
pub mod map;
pub mod menu;
pub mod tower;
pub mod tower_selection;
pub mod ui;

pub use bullet::bullet_movement;
pub use enemy::{enemy_movement, spawn_enemies, update_health_bars};
pub use input::{handle_build_input, update_mouse_position};
pub use laser::{laser_damage, laser_tower_targeting};
pub use map::setup_game;
pub use menu::{handle_menu_buttons, hide_menu, setup_menu};
pub use tower::tower_targeting;
pub use tower_selection::{handle_tower_selection, setup_tower_selection, update_selected_tower_indicator};
pub use ui::{update_console_ui, update_game_ui};