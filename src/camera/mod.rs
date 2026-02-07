pub mod collision;
pub mod third_person;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), third_person::spawn_camera)
            .add_systems(
                Update,
                (
                    third_person::orbit_camera,
                    collision::camera_collision,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
