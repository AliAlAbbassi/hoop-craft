use bevy::prelude::*;

use crate::elements::types::Element;

/// Marker for a playable character entity.
#[derive(Component)]
pub struct Character {
    pub name: String,
    pub element: Element,
}

/// Character stats used in combat calculations.
#[derive(Component, Clone)]
pub struct CharacterStats {
    pub max_hp: f32,
    pub current_hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub crit_rate: f32,
    pub crit_damage: f32,
    pub elemental_mastery: f32,
    pub energy_recharge: f32,
    pub max_energy: f32,
    pub current_energy: f32,
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self {
            max_hp: 1000.0,
            current_hp: 1000.0,
            attack: 100.0,
            defense: 50.0,
            crit_rate: 0.05,
            crit_damage: 0.5,
            elemental_mastery: 0.0,
            energy_recharge: 1.0,
            max_energy: 40.0,
            current_energy: 0.0,
        }
    }
}

/// Current animation state for the character.
#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AnimationState {
    #[default]
    Idle,
    Run,
    Sprint,
    Jump,
    Fall,
    Attack(u8), // combo index
    Skill,
    Burst,
    Stagger,
}

/// Marker for the currently active (player-controlled) character.
#[derive(Component)]
pub struct ActiveCharacter;

/// Velocity component for character movement (used before physics integration).
#[derive(Component, Default)]
pub struct CharacterVelocity {
    pub velocity: Vec3,
    pub grounded: bool,
}

/// Character's VRM model path.
#[derive(Component)]
pub struct VrmModelPath(pub String);
