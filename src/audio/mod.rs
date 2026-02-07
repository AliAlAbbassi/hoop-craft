pub mod systems;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlaySfx>()
            .add_systems(Startup, systems::load_audio_assets)
            .add_systems(
                Update,
                (
                    systems::start_bgm,
                    systems::play_sfx,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

/// SFX types.
#[derive(Debug, Clone, Copy)]
pub enum SfxKind {
    SwordSwing,
    Hit,
    SkillCast,
    BurstCast,
    PartySwap,
}

/// Message to trigger a sound effect.
#[derive(Message)]
pub struct PlaySfx {
    pub kind: SfxKind,
}

/// Holds all loaded audio asset handles.
#[derive(Resource)]
pub struct AudioAssets {
    pub bgm_battle: Handle<bevy_kira_audio::AudioSource>,
    pub sword_swings: Vec<Handle<bevy_kira_audio::AudioSource>>,
    pub hits: Vec<Handle<bevy_kira_audio::AudioSource>>,
    pub footsteps: Vec<Handle<bevy_kira_audio::AudioSource>>,
    pub bgm_started: bool,
}
