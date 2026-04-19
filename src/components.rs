use bevy::prelude::*;

/// 敌人组件
#[derive(Component)]
pub struct Enemy {
    pub speed: f32,         // 移动速度
    pub health: f32,        // 当前生命值
    pub max_health: f32,    // 最大生命值
    pub path_index: usize,   // 当前在路径上的索引
    pub bounty: u32,         // 被杀死后给予的赏金
}

/// 普通塔组件
#[derive(Component)]
pub struct Tower {
    pub range: f32,             // 攻击范围
    pub damage: f32,            // 攻击力
    pub attack_cooldown: Timer,  // 攻击冷却计时器
}

/// 子弹组件
#[derive(Component)]
pub struct Bullet {
    pub velocity: Vec2,    // 子弹速度
    pub damage: f32,       // 子弹伤害
}

/// 路径瓦片组件
#[derive(Component)]
pub struct PathTile;

/// 可建造的瓦片组件
#[derive(Component)]
pub struct BuildTile;

/// 金钱显示组件
#[derive(Component)]
pub struct MoneyDisplay;

/// 波次显示组件
#[derive(Component)]
pub struct WaveDisplay;

/// 血条组件
#[derive(Component)]
pub struct HealthBar {
    pub width: f32,             // 血条宽度
    pub height: f32,            // 血条高度
    pub enemy_entity: Entity,    // 关联的敌人实体
}

/// 激光塔组件
#[derive(Component)]
pub struct LaserTower {
    pub range: f32,             // 攻击范围
    pub damage: f32,            // 攻击力
    pub attack_cooldown: Timer,  // 攻击冷却计时器
}

/// 激光组件
#[derive(Component)]
pub struct Laser {
    pub target: Entity,    // 激光目标
    pub damage: f32,       // 激光伤害
    pub length: f32,       // 激光长度
}

/// 带护盾的敌人组件
#[derive(Component)]
pub struct ShieldedEnemy {
    pub has_shield: bool,  // 是否有护盾
}