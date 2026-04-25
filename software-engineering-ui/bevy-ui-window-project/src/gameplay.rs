use bevy::prelude::*;

#[derive(Event)]
pub struct BuildTowerEvent;

#[derive(Event)]
pub struct BuildResultEvent {
    pub success: bool,
}

#[derive(Event)]
pub struct TowerShootEvent;

#[derive(Event)]
pub struct EnemyDeathEvent;

#[derive(Resource)]
pub struct GameStats {
    pub gold: i32,
    pub health: i32,
    pub wave: u32,
}

#[derive(Resource)]
pub struct TowerState {
    pub count: u32,
}

#[derive(Resource)]
pub struct SimulationTimers {
    income_timer: Timer,
    wave_timer: Timer,
}

pub fn setup_gameplay(mut commands: Commands) {
    commands.insert_resource(GameStats {
        gold: 300,
        health: 20,
        wave: 1,
    });

    commands.insert_resource(TowerState {
        count: 0,
    });

    commands.insert_resource(SimulationTimers {
        income_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        wave_timer: Timer::from_seconds(12.0, TimerMode::Repeating),
    });
}

pub fn handle_build_tower_event(
    mut build_reader: EventReader<BuildTowerEvent>,
    mut result_writer: EventWriter<BuildResultEvent>,
    mut stats: ResMut<GameStats>,
    mut towers: ResMut<TowerState>,
) {
    for _ in build_reader.read() {
        if stats.gold >= 50 {
            stats.gold -= 50;
            towers.count += 1;
            result_writer.send(BuildResultEvent { success: true });
            info!("BUILD_TOWER: success, total towers = {}", towers.count);
        } else {
            result_writer.send(BuildResultEvent { success: false });
            info!("BUILD_TOWER: failed, not enough gold");
        }
    }
}

pub fn simulate_world_tick(time: Res<Time>, mut stats: ResMut<GameStats>, mut timers: ResMut<SimulationTimers>) {
    timers.income_timer.tick(time.delta());
    if timers.income_timer.just_finished() {
        stats.gold += 5;
    }

    timers.wave_timer.tick(time.delta());
    if timers.wave_timer.just_finished() {
        stats.wave += 1;
        if stats.health > 0 {
            stats.health -= 1;
        }
    }
}
