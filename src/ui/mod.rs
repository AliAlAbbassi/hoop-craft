pub mod action_bar;
pub mod damage_display;
pub mod hp_bar;
pub mod hud;
pub mod party_ui;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), hud::spawn_hud)
            .add_systems(
                Update,
                (
                    hp_bar::update_hp_bars,
                    action_bar::update_action_bar,
                    party_ui::update_party_portraits,
                    hud::update_character_name,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
