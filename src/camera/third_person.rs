use bevy::prelude::*;
use bevy::render::view::Hdr;

use crate::character::components::ActiveCharacter;
use crate::input::actions::InputActions;
use crate::rendering::post_processing::SobelOutlineEffect;

/// Orbit camera state.
#[derive(Component)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub target_offset: Vec3,
    pub sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: -20.0_f32.to_radians(),
            distance: 6.0,
            min_distance: 2.0,
            max_distance: 15.0,
            target_offset: Vec3::new(0.0, 1.2, 0.0),
            sensitivity: 0.003,
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Hdr,
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        OrbitCamera::default(),
        // Post-process sobel outlines (subtle)
        SobelOutlineEffect::new(0.3, Color::srgb(0.15, 0.1, 0.15), 1.0),
        // Bloom - pumped up for neon club glow
        bevy::post_process::bloom::Bloom {
            intensity: 0.15,
            ..default()
        },
    ));
}

pub fn orbit_camera(
    actions: Res<InputActions>,
    target_query: Query<&Transform, (With<ActiveCharacter>, Without<OrbitCamera>)>,
    mut camera_query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let Ok(target_tf) = target_query.single() else {
        return;
    };
    let Ok((mut cam_tf, mut orbit)) = camera_query.single_mut() else {
        return;
    };

    // Update angles from mouse
    orbit.yaw -= actions.look_delta.x * orbit.sensitivity;
    orbit.pitch -= actions.look_delta.y * orbit.sensitivity;
    orbit.pitch = orbit.pitch.clamp(-80.0_f32.to_radians(), 60.0_f32.to_radians());

    // Zoom from scroll
    orbit.distance -= actions.zoom_delta * 0.5;
    orbit.distance = orbit.distance.clamp(orbit.min_distance, orbit.max_distance);

    // Calculate camera position on sphere around target
    let target = target_tf.translation + orbit.target_offset;
    let rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
    let offset = rotation * Vec3::new(0.0, 0.0, orbit.distance);

    cam_tf.translation = target + offset;
    cam_tf.look_at(target, Vec3::Y);
}
