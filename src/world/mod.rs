pub mod terrain;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), terrain::spawn_terrain);
    }
}
