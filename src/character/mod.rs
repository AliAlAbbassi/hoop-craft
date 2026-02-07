pub mod animation;
pub mod components;
pub mod controller;
pub mod spawn;
pub mod weapon;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<spawn::SpawnCharacterEvent>()
            .add_systems(
                OnEnter(AppState::InGame),
                spawn::spawn_default_character,
            )
            .add_systems(
                Update,
                (
                    spawn::handle_spawn_events,
                    controller::character_movement,
                    controller::apply_gravity,
                    animation::update_animation_state,
                    weapon::spawn_weapon,
                    weapon::follow_active_character,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
