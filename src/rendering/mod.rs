pub mod cel_extension;
pub mod cel_material;
pub mod material_types;
pub mod outline_material;
pub mod post_processing;

use bevy::prelude::*;
use bevy::light::GlobalAmbientLight;

use crate::app_state::AppState;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cel_material::CelMaterialPlugin,
            outline_material::OutlineMaterialPlugin,
            post_processing::PostProcessPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.02, 0.01, 0.04)))
        .add_systems(OnEnter(AppState::InGame), setup_lighting);
    }
}

fn setup_lighting(mut commands: Commands, mut ambient: ResMut<GlobalAmbientLight>) {
    // Dim overhead light (simulates club ceiling fixtures)
    commands.spawn((
        DirectionalLight {
            illuminance: 800.0,
            shadows_enabled: true,
            color: Color::srgb(0.6, 0.5, 0.8),
            ..default()
        },
        Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, -80.0_f32.to_radians(), 0.0, 0.0),
        ),
    ));

    // Very dim ambient - club should be mostly lit by point lights and neons
    ambient.color = Color::srgb(0.3, 0.2, 0.4);
    ambient.brightness = 80.0;
}
