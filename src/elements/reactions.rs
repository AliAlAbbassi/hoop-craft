use bevy::prelude::*;

use super::aura::{ElementalApplicationEvent, ElementalAura};
use super::types::Element;

/// Types of elemental reactions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReactionType {
    // Amplifying (multiply damage)
    Vaporize { multiplier: f32 },
    Melt { multiplier: f32 },
    // Transformative (flat damage based on level + EM)
    Overloaded,
    Superconduct,
    ElectroCharged,
    Frozen,
    Shattered,
    Swirl { absorbed: Element },
    Crystallize { absorbed: Element },
    // Catalyze
    Aggravate,
    Spread,
    Bloom,
    Burning,
}

/// Message fired when an elemental reaction occurs.
#[derive(Message)]
pub struct ReactionEvent {
    pub target: Entity,
    pub reaction: ReactionType,
    pub trigger_element: Element,
}

/// Check for elemental reactions when a new element is applied to an entity with an existing aura.
pub fn check_reactions(
    mut commands: Commands,
    mut application_reader: MessageReader<ElementalApplicationEvent>,
    mut aura_query: Query<&mut ElementalAura>,
    mut reaction_writer: MessageWriter<ReactionEvent>,
) {
    for event in application_reader.read() {
        let Ok(mut aura) = aura_query.get_mut(event.target) else {
            continue;
        };

        // Same element: no reaction
        if aura.element == event.element {
            continue;
        }

        let reaction = match (aura.element, event.element) {
            // Vaporize: Hydro + Pyro (2x) or Pyro + Hydro (1.5x)
            (Element::Hydro, Element::Pyro) => Some(ReactionType::Vaporize { multiplier: 2.0 }),
            (Element::Pyro, Element::Hydro) => Some(ReactionType::Vaporize { multiplier: 1.5 }),

            // Melt: Cryo + Pyro (2x) or Pyro + Cryo (1.5x)
            (Element::Cryo, Element::Pyro) => Some(ReactionType::Melt { multiplier: 2.0 }),
            (Element::Pyro, Element::Cryo) => Some(ReactionType::Melt { multiplier: 1.5 }),

            // Overloaded: Pyro + Electro
            (Element::Pyro, Element::Electro) | (Element::Electro, Element::Pyro) => {
                Some(ReactionType::Overloaded)
            }

            // Superconduct: Cryo + Electro
            (Element::Cryo, Element::Electro) | (Element::Electro, Element::Cryo) => {
                Some(ReactionType::Superconduct)
            }

            // Electro-Charged: Hydro + Electro
            (Element::Hydro, Element::Electro) | (Element::Electro, Element::Hydro) => {
                Some(ReactionType::ElectroCharged)
            }

            // Frozen: Hydro + Cryo
            (Element::Hydro, Element::Cryo) | (Element::Cryo, Element::Hydro) => {
                Some(ReactionType::Frozen)
            }

            // Swirl: Anemo + any offensive element
            (existing, Element::Anemo) | (Element::Anemo, existing)
                if matches!(
                    existing,
                    Element::Pyro | Element::Hydro | Element::Electro | Element::Cryo
                ) =>
            {
                Some(ReactionType::Swirl { absorbed: existing })
            }

            // Crystallize: Geo + any offensive element
            (existing, Element::Geo) | (Element::Geo, existing)
                if matches!(
                    existing,
                    Element::Pyro | Element::Hydro | Element::Electro | Element::Cryo
                ) =>
            {
                Some(ReactionType::Crystallize { absorbed: existing })
            }

            // Burning: Pyro + Dendro
            (Element::Pyro, Element::Dendro) | (Element::Dendro, Element::Pyro) => {
                Some(ReactionType::Burning)
            }

            // Bloom: Hydro + Dendro
            (Element::Hydro, Element::Dendro) | (Element::Dendro, Element::Hydro) => {
                Some(ReactionType::Bloom)
            }

            // Aggravate: Electro on Quicken (Dendro + Electro)
            (Element::Dendro, Element::Electro) => Some(ReactionType::Aggravate),
            (Element::Electro, Element::Dendro) => Some(ReactionType::Spread),

            _ => None,
        };

        if let Some(reaction) = reaction {
            // Consume gauge based on reaction type
            let gauge_consumption = match &reaction {
                ReactionType::Vaporize { .. } | ReactionType::Melt { .. } => event.gauge_units * 0.5,
                ReactionType::Overloaded
                | ReactionType::Superconduct
                | ReactionType::Frozen
                | ReactionType::ElectroCharged => event.gauge_units,
                _ => event.gauge_units * 0.5,
            };

            aura.gauge -= gauge_consumption;
            if aura.gauge <= 0.0 {
                commands.entity(event.target).remove::<ElementalAura>();
            }

            reaction_writer.write(ReactionEvent {
                target: event.target,
                reaction,
                trigger_element: event.element,
            });
        }
    }
}
