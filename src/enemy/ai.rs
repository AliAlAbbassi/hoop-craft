use bevy::prelude::*;

use crate::character::components::ActiveCharacter;
use super::components::{AiState, Enemy};

const CHASE_RANGE: f32 = 15.0;
const ATTACK_RANGE: f32 = 2.0;
const MOVE_SPEED: f32 = 3.0;

/// Simple state machine AI for enemies.
pub fn enemy_ai(
    mut enemy_query: Query<(&mut Transform, &mut AiState, &Enemy), Without<ActiveCharacter>>,
    player_query: Query<&Transform, With<ActiveCharacter>>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_query.single() else {
        return;
    };

    for (mut tf, mut state, _enemy) in &mut enemy_query {
        let to_player = player_tf.translation - tf.translation;
        let dist = to_player.length();
        let dir = to_player.normalize_or_zero();

        match *state {
            AiState::Idle => {
                if dist < CHASE_RANGE {
                    *state = AiState::Chase;
                }
            }
            AiState::Chase => {
                if dist > CHASE_RANGE {
                    *state = AiState::Idle;
                } else if dist < ATTACK_RANGE {
                    *state = AiState::Attack;
                } else {
                    // Move toward player
                    let flat_dir = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                    tf.translation += flat_dir * MOVE_SPEED * time.delta_secs();

                    // Face player
                    if flat_dir.length_squared() > 0.01 {
                        tf.look_to(flat_dir, Vec3::Y);
                    }
                }
            }
            AiState::Attack => {
                if dist > ATTACK_RANGE * 1.5 {
                    *state = AiState::Chase;
                }
                // Attack logic handled by combat system
            }
            AiState::Stagger => {
                // Return to chase after stagger (simplified)
                *state = AiState::Chase;
            }
        }
    }
}

/// Remove enemies that have 0 HP.
pub fn remove_dead_enemies(
    mut commands: Commands,
    query: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in &query {
        if enemy.current_hp <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
