pub mod components;
pub mod switching;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct PartyPlugin;

impl Plugin for PartyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<components::Party>()
            .add_systems(
                Update,
                switching::handle_party_switching.run_if(in_state(AppState::InGame)),
            );
    }
}
