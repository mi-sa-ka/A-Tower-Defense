use bevy::prelude::*;

/// 塔类型枚举
#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum TowerType {
    None,   // 未选择
    Basic,  // 普通塔
    Laser,  // 激光塔
}

/// 游戏状态资源
#[derive(Resource)]
pub struct GameState {
    pub money: u32,              // 金钱
    pub lives: u32,              // 生命值
    pub wave: u32,               // 当前波次
    pub enemy_count: u32,         // 当前敌人数量
    pub selected_tower: TowerType, // 选中的塔类型
    pub has_spawned_enemies: bool, // 是否已经生成过敌人
    pub enemies_killed: u32,      // 杀死的敌人数量
    pub enemies_escaped: u32,     // 逃脱的敌人数量
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            money: 100,              // 初始金钱
            lives: 10,               // 初始生命值
            wave: 1,                // 初始波次
            enemy_count: 0,          // 初始敌人数量
            selected_tower: TowerType::None, // 初始未选择塔
            has_spawned_enemies: false, // 初始未生成敌人
            enemies_killed: 0,       // 初始杀死敌人数量
            enemies_escaped: 0,      // 初始逃脱敌人数量
        }
    }
}

/// 敌人路径资源
#[derive(Resource)]
pub struct EnemyPath {
    pub points: Vec<Vec2>, // 路径点列表
}

/// 敌人生成计时器资源
#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer, // 生成计时器
}

impl SpawnTimer {
    /// 创建新的生成计时器
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Repeating),
        }
    }
}

/// 鼠标位置资源
#[derive(Resource, Default)]
pub struct MousePosition {
    pub position: Option<Vec2>, // 鼠标在世界坐标中的位置
}

/// 游戏状态枚举
#[derive(Resource, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GameStatus {
    Menu,      // 菜单状态
    Playing,   // 游戏中状态
    GameOver,  // 游戏结束状态
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus::Menu
    }
}

/// 当前选中的塔实体（用于显示范围）
#[derive(Resource, Default)]
pub struct SelectedTower(pub Option<Entity>);