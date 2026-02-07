use bevy::prelude::*;

use crate::elements::types::Element;

/// Hitbox: attached to attack entities, deals damage on overlap.
#[derive(Component)]
pub struct HitBox {
    pub damage_multiplier: f32,
    pub element: Option<Element>,
    pub gauge_units: f32,
    pub owner: Entity,
    /// Entities already hit (prevent multi-hit per swing).
    pub hit_entities: Vec<Entity>,
}

/// Hurtbox: attached to damageable entities.
#[derive(Component)]
pub struct HurtBox;

/// Combo state tracking for normal attack chains.
#[derive(Component)]
pub struct ComboState {
    /// Current hit in the combo chain (0 = not attacking).
    pub current_hit: u8,
    /// Maximum hits in the chain.
    pub max_hits: u8,
    /// Time window to chain the next hit.
    pub chain_timer: Timer,
    /// Whether input was buffered during current animation.
    pub input_buffered: bool,
    /// Whether the current hit's active frames are done.
    pub hit_active: bool,
}

impl Default for ComboState {
    fn default() -> Self {
        Self {
            current_hit: 0,
            max_hits: 5,
            chain_timer: Timer::from_seconds(0.6, TimerMode::Once),
            input_buffered: false,
            hit_active: false,
        }
    }
}

/// Skill ability (E key).
#[derive(Component)]
pub struct Skill {
    pub cooldown: Timer,
    pub damage_multiplier: f32,
    pub element: Element,
    pub gauge_units: f32,
}

/// Burst ability (Q key).
#[derive(Component)]
pub struct Burst {
    pub energy_cost: f32,
    pub damage_multiplier: f32,
    pub element: Element,
    pub gauge_units: f32,
}
