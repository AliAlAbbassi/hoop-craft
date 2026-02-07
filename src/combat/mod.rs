pub mod combo_system;
pub mod components;
pub mod damage;
pub mod damage_numbers;
pub mod hit_detection;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<damage::DamageEvent>()
            .add_message::<damage::DamageResolvedEvent>()
            .add_systems(
                Update,
                (
                    combo_system::combo_input,
                    combo_system::skill_input,
                    combo_system::burst_input,
                    combo_system::tick_skill_cooldowns,
                    combo_system::advance_combo,
                    hit_detection::detect_hits,
                    damage::process_damage,
                    damage_numbers::spawn_damage_numbers,
                    damage_numbers::animate_damage_numbers,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
