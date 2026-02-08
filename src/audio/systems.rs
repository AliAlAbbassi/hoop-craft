use bevy::prelude::*;
use bevy_kira_audio::prelude::Audio;
use bevy_kira_audio::AudioControl;
use rand::seq::IndexedRandom;

use super::{AudioAssets, PlaySfx, SfxKind};

/// Load all audio assets at startup.
pub fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        sword_swings: vec![
            asset_server.load("audio/sfx/sword_swing1.ogg"),
            asset_server.load("audio/sfx/sword_swing2.ogg"),
            asset_server.load("audio/sfx/sword_swing3.ogg"),
        ],
        hits: vec![
            asset_server.load("audio/sfx/hit1.ogg"),
            asset_server.load("audio/sfx/hit2.ogg"),
            asset_server.load("audio/sfx/hit3.ogg"),
        ],
        footsteps: vec![
            asset_server.load("audio/sfx/footstep1.ogg"),
            asset_server.load("audio/sfx/footstep2.ogg"),
            asset_server.load("audio/sfx/footstep3.ogg"),
            asset_server.load("audio/sfx/footstep4.ogg"),
        ],
    });
}

/// Play sound effects based on messages.
pub fn play_sfx(
    mut reader: MessageReader<PlaySfx>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let mut rng = rand::rng();

    for event in reader.read() {
        let (pool, volume) = match event.kind {
            SfxKind::SwordSwing => (&audio_assets.sword_swings, 0.5),
            SfxKind::Hit => (&audio_assets.hits, 0.6),
            SfxKind::SkillCast => (&audio_assets.sword_swings, 0.7),
            SfxKind::BurstCast => (&audio_assets.hits, 0.8),
            SfxKind::PartySwap => (&audio_assets.hits, 0.3),
        };

        if let Some(handle) = pool.choose(&mut rng) {
            audio.play(handle.clone()).with_volume(volume);
        }
    }
}
