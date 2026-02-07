use bevy::prelude::*;

use super::types::Element;

/// Elemental aura applied to an entity (enemy or environment).
/// Uses a gauge unit system: aura decays over time, reactions consume gauge.
#[derive(Component)]
pub struct ElementalAura {
    pub element: Element,
    /// Remaining gauge units (starts at application value, decays over time).
    pub gauge: f32,
    /// Decay rate in gauge units per second.
    pub decay_rate: f32,
}

impl ElementalAura {
    pub fn new(element: Element, gauge: f32) -> Self {
        // Decay rate based on gauge amount (GU tax: higher gauge = slower decay)
        let decay_rate = match gauge {
            g if g <= 1.0 => 1.0 / 9.5,   // 1U: ~9.5s duration
            g if g <= 2.0 => 1.0 / 12.0,  // 2U: ~12s duration
            _ => 1.0 / 16.0,              // 4U: ~16s duration
        };
        Self {
            element,
            gauge,
            decay_rate,
        }
    }
}

/// Message to apply an element to a target.
#[derive(Message)]
pub struct ElementalApplicationEvent {
    pub target: Entity,
    pub element: Element,
    pub gauge_units: f32,
}

/// Apply elemental auras when elemental damage hits.
pub fn apply_elemental_aura(
    mut commands: Commands,
    mut reader: MessageReader<ElementalApplicationEvent>,
    mut existing_auras: Query<&mut ElementalAura>,
) {
    for event in reader.read() {
        if let Ok(mut aura) = existing_auras.get_mut(event.target) {
            if aura.element == event.element {
                // Same element: refresh gauge
                aura.gauge = (aura.gauge + event.gauge_units).min(4.0);
            }
            // Different element: reaction will be handled by reactions system
        } else {
            // No existing aura: apply new one
            commands
                .entity(event.target)
                .insert(ElementalAura::new(event.element, event.gauge_units));
        }
    }
}

/// Decay existing auras over time and remove when depleted.
pub fn decay_auras(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ElementalAura)>,
    time: Res<Time>,
) {
    for (entity, mut aura) in &mut query {
        aura.gauge -= aura.decay_rate * time.delta_secs();
        if aura.gauge <= 0.0 {
            commands.entity(entity).remove::<ElementalAura>();
        }
    }
}
