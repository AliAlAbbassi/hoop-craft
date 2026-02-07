use bevy::prelude::*;
use rand::Rng;

use crate::audio::{PlaySfx, SfxKind};
use crate::character::components::CharacterStats;
use crate::elements::aura::ElementalApplicationEvent;
use crate::elements::types::Element;
use crate::enemy::components::Enemy;

/// Message carrying raw damage data from a hit.
#[derive(Message)]
pub struct DamageEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub base_multiplier: f32,
    pub element: Option<Element>,
    pub gauge_units: f32,
}

/// Resolved damage after calculation.
#[derive(Message)]
pub struct DamageResolvedEvent {
    pub target: Entity,
    pub amount: f32,
    pub is_crit: bool,
    pub element: Option<Element>,
    pub position: Vec3,
}

/// Calculate and apply damage.
pub fn process_damage(
    mut damage_reader: MessageReader<DamageEvent>,
    mut resolved_writer: MessageWriter<DamageResolvedEvent>,
    mut elemental_writer: MessageWriter<ElementalApplicationEvent>,
    mut sfx_writer: MessageWriter<PlaySfx>,
    attacker_stats: Query<&CharacterStats>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
) {
    let mut rng = rand::rng();

    for event in damage_reader.read() {
        let Ok(stats) = attacker_stats.get(event.attacker) else {
            continue;
        };

        // Base damage = ATK * multiplier
        let base_damage = stats.attack * event.base_multiplier;

        // Crit check
        let is_crit = rng.random::<f32>() < stats.crit_rate;
        let crit_mult = if is_crit {
            1.0 + stats.crit_damage
        } else {
            1.0
        };

        // DEF reduction (simplified)
        let def_factor = if let Ok((_, enemy)) = enemy_query.get(event.target) {
            100.0 / (100.0 + enemy.defense)
        } else {
            1.0
        };

        let final_damage = base_damage * crit_mult * def_factor;

        // Apply damage to enemy HP
        let position = if let Ok((tf, mut enemy)) = enemy_query.get_mut(event.target) {
            enemy.current_hp = (enemy.current_hp - final_damage).max(0.0);
            tf.translation + Vec3::Y * 1.5
        } else {
            Vec3::ZERO
        };

        sfx_writer.write(PlaySfx { kind: SfxKind::Hit });

        resolved_writer.write(DamageResolvedEvent {
            target: event.target,
            amount: final_damage,
            is_crit,
            element: event.element,
            position,
        });

        // Apply elemental aura if the attack has an element
        if let Some(element) = event.element {
            elemental_writer.write(ElementalApplicationEvent {
                target: event.target,
                element,
                gauge_units: event.gauge_units,
            });
        }
    }
}
