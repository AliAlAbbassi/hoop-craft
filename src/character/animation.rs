use bevy::prelude::*;

use super::components::{AnimationState, Character};

/// Timer for animation state transitions (e.g., attack recovery).
#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub return_to: AnimationState,
}

/// Update animation-related logic (timers, transitions).
pub fn update_animation_state(
    mut commands: Commands,
    mut query: Query<(Entity, &mut AnimationState, Option<&mut AnimationTimer>), With<Character>>,
    time: Res<Time>,
) {
    for (entity, mut anim, timer) in &mut query {
        if let Some(mut timer) = timer {
            timer.timer.tick(time.delta());
            if timer.timer.is_finished() {
                *anim = timer.return_to;
                commands.entity(entity).remove::<AnimationTimer>();
            }
        }
    }
}
