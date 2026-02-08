#![allow(dead_code)]

mod app_state;
mod audio;
mod audio_reactive;
mod camera;
mod character;
mod combat;
mod elements;
mod enemy;
mod input;
mod loading;
mod party;
mod rendering;
mod stamina;
mod ui;
mod world;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use avian3d::prelude::*;
use bevy_vrm1::prelude::*;
use bevy_kira_audio::AudioPlugin;

use app_state::AppState;

fn main() {
    App::new()
        // Core Bevy
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hoop Craft".to_string(),
                resolution: WindowResolution::new(1280, 720),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        // State
        .init_state::<AppState>()
        // Third-party plugins
        .add_plugins((
            PhysicsPlugins::default(),
            VrmPlugin,
            AudioPlugin,
        ))
        // Game plugins
        .add_plugins((
            loading::LoadingPlugin,
            input::InputPlugin,
            rendering::RenderingPlugin,
            camera::CameraPlugin,
            character::CharacterPlugin,
            combat::CombatPlugin,
            elements::ElementPlugin,
            party::PartyPlugin,
            stamina::StaminaPlugin,
            enemy::EnemyPlugin,
            world::WorldPlugin,
            ui::UiPlugin,
            audio::GameAudioPlugin,
            audio_reactive::AudioReactivePlugin,
        ))
        .run();
}
