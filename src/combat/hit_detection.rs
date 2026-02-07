use avian3d::prelude::*;
use bevy::prelude::*;

use super::components::{HitBox, HurtBox};
use super::damage::DamageEvent;

/// Detect hitbox/hurtbox overlaps using Avian collision events.
pub fn detect_hits(
    mut collision_starts: MessageReader<CollisionStart>,
    mut hitboxes: Query<&mut HitBox>,
    hurtboxes: Query<Entity, With<HurtBox>>,
    mut damage_writer: MessageWriter<DamageEvent>,
) {
    for event in collision_starts.read() {
        let entity_a = event.collider1;
        let entity_b = event.collider2;

        // Check both orderings: A=hitbox B=hurtbox and vice versa
        let (hitbox_entity, hurtbox_entity) =
            if hitboxes.contains(entity_a) && hurtboxes.contains(entity_b) {
                (entity_a, entity_b)
            } else if hitboxes.contains(entity_b) && hurtboxes.contains(entity_a) {
                (entity_b, entity_a)
            } else {
                continue;
            };

        let Ok(mut hitbox) = hitboxes.get_mut(hitbox_entity) else {
            continue;
        };

        // Skip if already hit this entity
        if hitbox.hit_entities.contains(&hurtbox_entity) {
            continue;
        }

        // Skip self-damage
        if hitbox.owner == hurtbox_entity {
            continue;
        }

        hitbox.hit_entities.push(hurtbox_entity);

        damage_writer.write(DamageEvent {
            attacker: hitbox.owner,
            target: hurtbox_entity,
            base_multiplier: hitbox.damage_multiplier,
            element: hitbox.element,
            gauge_units: hitbox.gauge_units,
        });
    }
}
