pub mod aura;
pub mod reactions;
pub mod types;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct ElementPlugin;

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<aura::ElementalApplicationEvent>()
            .add_message::<reactions::ReactionEvent>()
            .add_systems(
                Update,
                (
                    aura::apply_elemental_aura,
                    aura::decay_auras,
                    reactions::check_reactions,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
