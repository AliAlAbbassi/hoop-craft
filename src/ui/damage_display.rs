// Damage number rendering is handled by combat/damage_numbers.rs
// This module is reserved for additional damage display UI (target frame, etc.)

use bevy::prelude::*;

/// Marker for the target info frame UI.
#[derive(Component)]
pub struct TargetFrame;

/// Marker for target HP bar.
#[derive(Component)]
pub struct TargetHpBar;

/// Currently targeted enemy (for UI display).
#[derive(Resource, Default)]
pub struct CurrentTarget {
    pub entity: Option<Entity>,
}
