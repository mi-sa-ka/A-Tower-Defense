use bevy::prelude::*;

use crate::gameplay::{EnemyDeathEvent, TowerShootEvent};

#[derive(Resource)]
pub struct AudioAssets {
    pub bgm: Handle<AudioSource>,
    pub pew: Handle<AudioSource>,
    pub explode: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct AudioSettings {
    pub enabled: bool,
}

#[derive(Component)]
pub struct BgmAudio;

pub fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        bgm: asset_server.load("audio/bgm.wav"),
        pew: asset_server.load("audio/pew.wav"),
        explode: asset_server.load("audio/explode.wav"),
    });

    commands.insert_resource(AudioSettings { enabled: true });
}

pub fn start_bgm(
    mut commands: Commands,
    assets: Option<Res<AudioAssets>>,
    settings: Option<Res<AudioSettings>>,
) {
    let Some(assets) = assets else {
        warn!("AUDIO: assets not initialized, skip start_bgm");
        return;
    };
    let Some(settings) = settings else {
        warn!("AUDIO: settings not initialized, skip start_bgm");
        return;
    };

    if !settings.enabled {
        return;
    }

    commands.spawn((
        AudioPlayer::new(assets.bgm.clone()),
        PlaybackSettings::LOOP,
        BgmAudio,
    ));
}

pub fn toggle_audio_system(
    input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<AudioSettings>,
    assets: Res<AudioAssets>,
    mut commands: Commands,
    bgm_query: Query<Entity, With<BgmAudio>>,
) {
    if !input.just_pressed(KeyCode::KeyM) {
        return;
    }

    settings.enabled = !settings.enabled;

    if settings.enabled {
        if bgm_query.is_empty() {
            commands.spawn((
                AudioPlayer::new(assets.bgm.clone()),
                PlaybackSettings::LOOP,
                BgmAudio,
            ));
        }
        info!("AUDIO: enabled");
    } else {
        for entity in &bgm_query {
            commands.entity(entity).despawn();
        }
        info!("AUDIO: disabled");
    }
}

pub fn play_shoot_sfx(
    mut reader: EventReader<TowerShootEvent>,
    settings: Res<AudioSettings>,
    assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    if !settings.enabled {
        return;
    }

    for _ in reader.read() {
        commands.spawn((
            AudioPlayer::new(assets.pew.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub fn play_explode_sfx(
    mut reader: EventReader<EnemyDeathEvent>,
    settings: Res<AudioSettings>,
    assets: Res<AudioAssets>,
    mut commands: Commands,
) {
    if !settings.enabled {
        return;
    }

    for _ in reader.read() {
        commands.spawn((
            AudioPlayer::new(assets.explode.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
