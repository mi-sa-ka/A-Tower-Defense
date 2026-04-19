// 模块导入
mod components;    // 组件定义
mod constants;    // 常量定义
mod resources;    // 资源定义
mod systems;      // 系统实现

// 外部库导入
use bevy::prelude::*;
use constants::{MAP_HEIGHT, MAP_WIDTH, SPAWN_INTERVAL, TILE_SIZE};
use resources::{EnemyPath, GameState, GameStatus, MousePosition, SpawnTimer};
use systems::{
    // 敌人系统
    enemy_movement, spawn_enemies, update_health_bars,
    // 塔系统
    tower_targeting, laser_tower_targeting, laser_damage, bullet_movement,
    // 输入系统
    update_mouse_position, handle_build_input,
    // UI系统
    setup_menu, handle_menu_buttons, hide_menu, update_console_ui, update_game_ui,
    // 塔选择系统
    setup_tower_selection, handle_tower_selection, update_selected_tower_indicator,
    // 游戏设置
    setup_game,
};

/// 游戏主函数
fn main() {
    App::new()
        // 添加默认插件
        .add_plugins(DefaultPlugins)
        // 插入游戏状态资源
        .insert_resource(GameState::default())
        // 插入敌人路径资源
        .insert_resource(EnemyPath {
            points: vec![
                // 敌人路径点定义
                Vec2::new(
                    0.0 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                    6.0 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                ),
                Vec2::new(
                    5.0 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                    6.0 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                ),
                Vec2::new(
                    5.0 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                    9.0 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                ),
                Vec2::new(
                    10.0 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                    9.0 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                ),
                Vec2::new(
                    10.0 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                    3.0 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                ),
                Vec2::new(
                    14.0 * TILE_SIZE - (MAP_WIDTH as f32 * TILE_SIZE) / 2.0,
                    3.0 * TILE_SIZE - (MAP_HEIGHT as f32 * TILE_SIZE) / 2.0,
                ),
            ],
        })
        // 插入敌人生成计时器
        .insert_resource(SpawnTimer::new(SPAWN_INTERVAL))
        // 插入鼠标位置资源
        .insert_resource(MousePosition::default())
        // 插入游戏状态资源，初始为菜单状态
        .insert_resource(GameStatus::Menu)
        
        // 添加系统
        // 菜单系统
        .add_system(handle_menu_buttons)    // 处理菜单按钮点击
        .add_system(setup_menu)             // 设置菜单UI
        .add_system(hide_menu)              // 隐藏菜单
        
        // 游戏设置
        .add_system(setup_game)             // 设置游戏地图和UI
        
        // 敌人系统
        .add_system(enemy_movement)         // 敌人移动
        .add_system(spawn_enemies)          // 生成敌人
        .add_system(update_health_bars)     // 更新敌人血条
        
        // 塔系统
        .add_system(tower_targeting)        // 普通塔索敌
        .add_system(laser_tower_targeting)  // 激光塔索敌
        .add_system(laser_damage)           // 激光伤害
        .add_system(bullet_movement)        // 子弹移动
        
        // 输入系统
        .add_system(update_mouse_position)  // 更新鼠标位置
        .add_system(handle_build_input)      // 处理建造输入
        
        // UI系统
        .add_system(update_console_ui)       // 更新控制台UI
        .add_system(update_game_ui)          // 更新游戏UI
        
        // 塔选择系统
        .add_system(setup_tower_selection)  // 设置塔选择UI
        .add_system(handle_tower_selection) // 处理塔选择
        .add_system(update_selected_tower_indicator) // 更新选中塔的指示器
        
        // 运行游戏
        .run();
}