pub mod ai;
pub mod components;
pub mod spawner;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawner::spawn_test_enemies)
            .add_systems(
                Update,
                (
                    ai::enemy_ai,
                    ai::remove_dead_enemies,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
