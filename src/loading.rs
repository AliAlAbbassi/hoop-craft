use bevy::prelude::*;

use crate::app_state::AppState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingTracker>()
            .add_systems(OnEnter(AppState::Loading), start_loading)
            .add_systems(Update, check_loading.run_if(in_state(AppState::Loading)));
    }
}

/// Tracks asset loading progress.
#[derive(Resource, Default)]
pub struct LoadingTracker {
    pub handles: Vec<UntypedHandle>,
    pub done: bool,
}

fn start_loading(mut tracker: ResMut<LoadingTracker>) {
    // Assets will be registered by other systems during Loading state.
    // For now, mark as ready to proceed.
    tracker.done = true;
}

fn check_loading(
    tracker: Res<LoadingTracker>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if tracker.done {
        let all_loaded = tracker.handles.iter().all(|h| {
            asset_server
                .get_load_state(h.id())
                .is_some_and(|s| s.is_loaded())
        });
        if all_loaded {
            info!("All assets loaded, transitioning to InGame");
            next_state.set(AppState::InGame);
        }
    }
}
