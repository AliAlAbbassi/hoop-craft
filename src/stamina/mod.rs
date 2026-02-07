pub mod systems;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct StaminaPlugin;

impl Plugin for StaminaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::Stamina>()
            .add_systems(
                Update,
                systems::update_stamina.run_if(in_state(AppState::InGame)),
            );
    }
}
