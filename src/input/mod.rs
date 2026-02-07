pub mod actions;

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<actions::InputActions>()
            .add_systems(Update, actions::update_input_actions);
    }
}
