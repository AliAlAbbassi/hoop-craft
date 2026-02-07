use bevy::prelude::*;
use bevy::pbr::ExtendedMaterial;

use super::cel_extension::CelExtension;

/// The main cel-shaded material type: StandardMaterial + cel-shading extension.
pub type CelMaterial = ExtendedMaterial<StandardMaterial, CelExtension>;

pub struct CelMaterialPlugin;

impl Plugin for CelMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CelMaterial>::default());
    }
}
