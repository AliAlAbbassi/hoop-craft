use bevy::prelude::*;

use crate::audio::{PlaySfx, SfxKind};
use crate::character::animation::AnimationTimer;
use crate::character::components::*;
use crate::input::actions::InputActions;
use super::components::ComboState;

/// Damage multipliers per combo hit index.
const COMBO_MULTIPLIERS: [f32; 5] = [1.0, 1.1, 1.2, 0.8, 1.5];
/// Duration of each combo hit animation in seconds.
const COMBO_DURATIONS: [f32; 5] = [0.35, 0.3, 0.35, 0.25, 0.5];

/// Read attack input and buffer it into the combo system.
pub fn combo_input(
    actions: Res<InputActions>,
    mut query: Query<&mut ComboState, With<ActiveCharacter>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    let Ok(mut combo) = query.single_mut() else {
        return;
    };

    if actions.attack {
        if combo.current_hit == 0 {
            // Start combo
            combo.current_hit = 1;
            combo.hit_active = true;
            combo
                .chain_timer
                .set_duration(std::time::Duration::from_secs_f32(
                    COMBO_DURATIONS[0],
                ));
            combo.chain_timer.reset();
            sfx.write(PlaySfx { kind: SfxKind::SwordSwing });
        } else {
            // Buffer next hit
            combo.input_buffered = true;
        }
    }
}

/// Advance the combo chain based on timing windows.
pub fn advance_combo(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut ComboState, &mut AnimationState),
        With<ActiveCharacter>,
    >,
    time: Res<Time>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    let Ok((entity, mut combo, mut anim)) = query.single_mut() else {
        return;
    };

    if combo.current_hit == 0 {
        return;
    }

    combo.chain_timer.tick(time.delta());

    if combo.chain_timer.is_finished() {
        if combo.input_buffered && combo.current_hit < combo.max_hits {
            // Chain to next hit
            combo.current_hit += 1;
            combo.input_buffered = false;
            combo.hit_active = true;

            let idx = (combo.current_hit - 1) as usize;
            let duration = if idx < COMBO_DURATIONS.len() {
                COMBO_DURATIONS[idx]
            } else {
                0.4
            };

            combo
                .chain_timer
                .set_duration(std::time::Duration::from_secs_f32(duration));
            combo.chain_timer.reset();

            *anim = AnimationState::Attack(combo.current_hit);
            sfx.write(PlaySfx { kind: SfxKind::SwordSwing });
        } else {
            // Combo ended: reset
            combo.current_hit = 0;
            combo.input_buffered = false;
            combo.hit_active = false;

            // Return to idle after recovery
            commands.entity(entity).insert(AnimationTimer {
                timer: Timer::from_seconds(0.15, TimerMode::Once),
                return_to: AnimationState::Idle,
            });
        }
    } else if combo.current_hit > 0 {
        *anim = AnimationState::Attack(combo.current_hit);
    }
}

/// Handle skill (E) input.
pub fn skill_input(
    actions: Res<InputActions>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationState, &mut ComboState, &mut super::components::Skill), With<ActiveCharacter>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    let Ok((entity, mut anim, mut combo, mut skill)) = query.single_mut() else {
        return;
    };

    if actions.skill && skill.cooldown.is_finished() {
        // Cancel any combo
        combo.current_hit = 0;
        combo.input_buffered = false;
        combo.hit_active = false;

        *anim = AnimationState::Skill;
        skill.cooldown.reset();
        sfx.write(PlaySfx { kind: SfxKind::SkillCast });

        // Return to idle after skill animation
        commands.entity(entity).insert(AnimationTimer {
            timer: Timer::from_seconds(0.8, TimerMode::Once),
            return_to: AnimationState::Idle,
        });
    }
}

/// Handle burst (Q) input.
pub fn burst_input(
    actions: Res<InputActions>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationState, &mut ComboState), With<ActiveCharacter>>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    let Ok((entity, mut anim, mut combo)) = query.single_mut() else {
        return;
    };

    if actions.burst {
        // Cancel any combo
        combo.current_hit = 0;
        combo.input_buffered = false;
        combo.hit_active = false;

        *anim = AnimationState::Burst;
        sfx.write(PlaySfx { kind: SfxKind::BurstCast });

        // Return to idle after burst animation
        commands.entity(entity).insert(AnimationTimer {
            timer: Timer::from_seconds(1.5, TimerMode::Once),
            return_to: AnimationState::Idle,
        });
    }
}

/// Tick skill cooldowns for all characters.
pub fn tick_skill_cooldowns(
    mut query: Query<&mut super::components::Skill>,
    time: Res<Time>,
) {
    for mut skill in &mut query {
        skill.cooldown.tick(time.delta());
    }
}

/// Get the damage multiplier for a given combo hit.
pub fn combo_multiplier(hit: u8) -> f32 {
    let idx = (hit.saturating_sub(1)) as usize;
    if idx < COMBO_MULTIPLIERS.len() {
        COMBO_MULTIPLIERS[idx]
    } else {
        1.0
    }
}
