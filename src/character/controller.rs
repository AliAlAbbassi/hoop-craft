use bevy::prelude::*;

use crate::input::actions::InputActions;
use crate::stamina::systems::Stamina;
use super::components::*;

const MOVE_SPEED: f32 = 5.0;
const SPRINT_SPEED: f32 = 8.0;
const JUMP_FORCE: f32 = 8.0;
const GRAVITY: f32 = -20.0;

/// Move the active character based on input, relative to camera facing.
pub fn character_movement(
    actions: Res<InputActions>,
    camera_q: Query<&Transform, (With<Camera3d>, Without<ActiveCharacter>)>,
    mut char_q: Query<
        (&mut Transform, &mut CharacterVelocity, &mut AnimationState),
        With<ActiveCharacter>,
    >,
    stamina: Option<Res<Stamina>>,
    time: Res<Time>,
) {
    let Ok(camera_tf) = camera_q.single() else {
        return;
    };
    let Ok((mut tf, mut vel, mut anim)) = char_q.single_mut() else {
        return;
    };

    // Camera-relative movement direction
    let cam_forward = camera_tf.forward().as_vec3();
    let cam_forward_flat = Vec3::new(cam_forward.x, 0.0, cam_forward.z).normalize_or_zero();
    let cam_right = Vec3::new(-cam_forward_flat.z, 0.0, cam_forward_flat.x);

    let move_dir =
        cam_forward_flat * actions.movement.y + cam_right * actions.movement.x;

    let can_sprint = actions.sprint
        && stamina.as_ref().is_none_or(|s| s.current > 0.0);

    let speed = if can_sprint { SPRINT_SPEED } else { MOVE_SPEED };
    let horizontal = move_dir * speed;

    // Jump
    if actions.jump && vel.grounded {
        vel.velocity.y = JUMP_FORCE;
        vel.grounded = false;
    }

    // Apply horizontal movement
    vel.velocity.x = horizontal.x;
    vel.velocity.z = horizontal.z;

    // Apply position
    let delta = vel.velocity * time.delta_secs();
    tf.translation += delta;

    // Face movement direction
    if move_dir.length_squared() > 0.01 {
        let target_rotation = Quat::from_rotation_arc(Vec3::Z, move_dir.normalize());
        tf.rotation = tf.rotation.slerp(target_rotation, 10.0 * time.delta_secs());
    }

    // Update animation state
    let new_anim = if !vel.grounded && vel.velocity.y > 0.5 {
        AnimationState::Jump
    } else if !vel.grounded {
        AnimationState::Fall
    } else if move_dir.length_squared() > 0.01 {
        if can_sprint {
            AnimationState::Sprint
        } else {
            AnimationState::Run
        }
    } else {
        AnimationState::Idle
    };

    // Don't override attack/skill/burst animations
    match *anim {
        AnimationState::Attack(_) | AnimationState::Skill | AnimationState::Burst => {}
        _ => *anim = new_anim,
    }
}

/// Simple gravity + ground detection.
pub fn apply_gravity(
    mut query: Query<(&mut CharacterVelocity, &Transform), With<Character>>,
    time: Res<Time>,
) {
    for (mut vel, tf) in &mut query {
        // Apply gravity
        vel.velocity.y += GRAVITY * time.delta_secs();

        // Simple ground plane at y=0 (capsule bottom ~0.8 below center)
        let ground_y = 0.8;
        if tf.translation.y <= ground_y {
            vel.velocity.y = vel.velocity.y.max(0.0);
            vel.grounded = true;
        } else {
            vel.grounded = false;
        }
    }
}
