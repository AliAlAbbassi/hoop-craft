use bevy::prelude::*;

use crate::character::components::{ActiveCharacter, AnimationState};
use crate::input::actions::InputActions;

const MAX_STAMINA: f32 = 100.0;
const SPRINT_DRAIN: f32 = 15.0;   // per second
const CLIMB_DRAIN: f32 = 10.0;    // per second
const GLIDE_DRAIN: f32 = 5.0;     // per second
const REGEN_RATE: f32 = 25.0;     // per second
const REGEN_DELAY: f32 = 1.0;     // seconds after draining before regen starts

/// Global stamina resource shared across party.
#[derive(Resource)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_delay_timer: Timer,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: MAX_STAMINA,
            max: MAX_STAMINA,
            regen_delay_timer: Timer::from_seconds(REGEN_DELAY, TimerMode::Once),
        }
    }
}

pub fn update_stamina(
    mut stamina: ResMut<Stamina>,
    actions: Res<InputActions>,
    active_query: Query<&AnimationState, With<ActiveCharacter>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let mut draining = false;

    let anim_state = active_query.single().ok().copied();

    if actions.sprint && anim_state.is_some_and(|a| a == AnimationState::Sprint) {
        stamina.current -= SPRINT_DRAIN * dt;
        draining = true;
    }

    // Future: climb / glide drain here
    let _ = (CLIMB_DRAIN, GLIDE_DRAIN);

    stamina.current = stamina.current.clamp(0.0, stamina.max);

    if draining {
        stamina.regen_delay_timer.reset();
    } else {
        stamina.regen_delay_timer.tick(time.delta());
        if stamina.regen_delay_timer.is_finished() {
            stamina.current = (stamina.current + REGEN_RATE * dt).min(stamina.max);
        }
    }
}
