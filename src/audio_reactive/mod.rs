use bevy::prelude::*;

use crate::app_state::AppState;

// ─── FFI to Objective-C bridge ───
unsafe extern "C" {
    safe fn audio_capture_start();
    safe fn audio_capture_stop();
    safe fn audio_capture_is_active() -> i32;
    safe fn audio_capture_get_bass() -> f32;
    safe fn audio_capture_get_mid() -> f32;
    safe fn audio_capture_get_treble() -> f32;
    safe fn audio_capture_get_volume() -> f32;
}

// ─── Marker components for reactive entities ───

#[derive(Component)]
pub struct ReactiveFloor {
    pub base_emissive: LinearRgba,
}

#[derive(Component)]
pub struct ReactiveSpotlight {
    pub base_intensity: f32,
}

#[derive(Component)]
pub struct ReactiveNeon {
    pub base_emissive: LinearRgba,
}

// ─── Audio levels resource ───

#[derive(Resource)]
pub struct AudioLevels {
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
    pub volume: f32,
    // Smoothed values (for visuals)
    pub smooth_bass: f32,
    pub smooth_mid: f32,
    pub smooth_treble: f32,
    pub smooth_volume: f32,
}

impl Default for AudioLevels {
    fn default() -> Self {
        Self {
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            volume: 0.0,
            smooth_bass: 0.0,
            smooth_mid: 0.0,
            smooth_treble: 0.0,
            smooth_volume: 0.0,
        }
    }
}

// ─── Plugin ───

pub struct AudioReactivePlugin;

impl Plugin for AudioReactivePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioLevels>()
            .add_systems(OnEnter(AppState::InGame), start_capture)
            .add_systems(
                Update,
                (
                    poll_audio_levels,
                    react_floor_tiles,
                    react_spotlights,
                    react_neons,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn start_capture() {
    info!("Starting system audio capture via ScreenCaptureKit...");
    audio_capture_start();
}

fn poll_audio_levels(mut levels: ResMut<AudioLevels>) {
    if audio_capture_is_active() == 0 {
        return;
    }

    let raw_bass = audio_capture_get_bass();
    let raw_mid = audio_capture_get_mid();
    let raw_treble = audio_capture_get_treble();
    let raw_volume = audio_capture_get_volume();

    levels.bass = raw_bass;
    levels.mid = raw_mid;
    levels.treble = raw_treble;
    levels.volume = raw_volume;

    // Exponential smoothing (fast attack, slower decay)
    let attack = 0.6;
    let decay = 0.15;

    fn smooth(current: f32, target: f32, attack: f32, decay: f32) -> f32 {
        let factor = if target > current { attack } else { decay };
        current + (target - current) * factor
    }

    levels.smooth_bass = smooth(levels.smooth_bass, raw_bass, attack, decay);
    levels.smooth_mid = smooth(levels.smooth_mid, raw_mid, attack, decay);
    levels.smooth_treble = smooth(levels.smooth_treble, raw_treble, attack, decay);
    levels.smooth_volume = smooth(levels.smooth_volume, raw_volume, attack, decay);
}

fn react_floor_tiles(
    levels: Res<AudioLevels>,
    tiles: Query<(&ReactiveFloor, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let bass = levels.smooth_bass;

    for (floor, mat_handle) in &tiles {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            // Scale emissive brightness with bass
            let intensity = 1.0 + bass * 6.0;
            mat.emissive = LinearRgba::new(
                floor.base_emissive.red * intensity,
                floor.base_emissive.green * intensity,
                floor.base_emissive.blue * intensity,
                1.0,
            );
        }
    }
}

fn react_spotlights(
    levels: Res<AudioLevels>,
    mut lights: Query<(&ReactiveSpotlight, &mut PointLight)>,
) {
    let mid = levels.smooth_mid;

    for (spot, mut light) in &mut lights {
        // Pulse intensity with mid frequencies
        light.intensity = spot.base_intensity * (1.0 + mid * 3.0);
    }
}

fn react_neons(
    levels: Res<AudioLevels>,
    neons: Query<(&ReactiveNeon, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let treble = levels.smooth_treble;

    for (neon, mat_handle) in &neons {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            let intensity = 1.0 + treble * 4.0;
            mat.emissive = LinearRgba::new(
                neon.base_emissive.red * intensity,
                neon.base_emissive.green * intensity,
                neon.base_emissive.blue * intensity,
                1.0,
            );
        }
    }
}
