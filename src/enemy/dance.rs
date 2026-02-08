use avian3d::prelude::*;
use bevy::prelude::*;

use crate::audio_reactive::AudioLevels;
use super::components::{AiState, Enemy};

/// Attached to enemies so they dance when idle.
#[derive(Component)]
pub struct Dancer {
    pub phase: f32,
    pub base_y: f32,
    pub time: f32,
}

/// Attach a Dancer component to every enemy once they exist.
pub fn init_dancers(
    mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Enemy>, Without<Dancer>)>,
) {
    for (i, (entity, tf)) in query.iter().enumerate() {
        commands.entity(entity).insert(Dancer {
            phase: i as f32 * 1.3, // stagger so they don't all sync
            base_y: tf.translation.y,
            time: 0.0,
        });
    }
}

/// Make idle enemies dance. Bobbing via LinearVelocity, swaying via Transform rotation.
pub fn dance_system(
    mut query: Query<
        (&mut Dancer, &mut Transform, &mut LinearVelocity, &AiState),
        With<Enemy>,
    >,
    time: Res<Time>,
    levels: Res<AudioLevels>,
) {
    let dt = time.delta_secs();

    for (mut dancer, mut tf, mut vel, state) in &mut query {
        dancer.time += dt;

        if *state != AiState::Idle {
            // Not dancing — reset rotation to upright
            tf.rotation = tf.rotation.slerp(Quat::IDENTITY, dt * 5.0);
            continue;
        }

        let t = dancer.time + dancer.phase;
        let bass = levels.smooth_bass;
        let mid = levels.smooth_mid;

        // ── Bob up and down (bass drives intensity) ──
        let bob_strength = 2.0 + bass * 8.0;
        let bob_freq = 3.0 + bass * 2.0;
        vel.0.y = (t * bob_freq).sin() * bob_strength;

        // ── Sway side to side (mid drives intensity) ──
        let sway_angle = (t * 2.0).sin() * (0.08 + mid * 0.15);
        let tilt_angle = (t * 1.5 + 0.5).cos() * (0.05 + mid * 0.1);

        // ── Spin slowly ──
        let spin = (t * 0.4).sin() * 0.3;

        let target = Quat::from_euler(
            EulerRot::XYZ,
            tilt_angle,  // forward/back tilt
            spin,        // spin
            sway_angle,  // side sway
        );

        tf.rotation = tf.rotation.slerp(target, dt * 6.0);
    }
}
