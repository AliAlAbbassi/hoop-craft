use bevy::prelude::*;

use crate::audio::{PlaySfx, SfxKind};
use crate::character::components::ActiveCharacter;
use crate::input::actions::InputActions;
use super::components::Party;

/// Handle 1-4 key presses to swap active character with 1s cooldown.
pub fn handle_party_switching(
    mut commands: Commands,
    mut party: ResMut<Party>,
    actions: Res<InputActions>,
    active_query: Query<Entity, With<ActiveCharacter>>,
    mut visibility_query: Query<&mut Visibility>,
    time: Res<Time>,
    mut sfx: MessageWriter<PlaySfx>,
) {
    party.switch_cooldown.tick(time.delta());

    let Some(slot_index) = actions.party_slot else {
        return;
    };

    // Can't switch to same character or on cooldown
    if slot_index == party.active_index || !party.switch_cooldown.is_finished() {
        return;
    }

    // Must have a character in the target slot
    let Some(new_entity) = party.slots[slot_index] else {
        return;
    };

    // Hide current active character
    if let Ok(current) = active_query.single() {
        commands.entity(current).remove::<ActiveCharacter>();
        if let Ok(mut vis) = visibility_query.get_mut(current) {
            *vis = Visibility::Hidden;
        }
    }

    // Show and activate new character
    commands.entity(new_entity).insert(ActiveCharacter);
    if let Ok(mut vis) = visibility_query.get_mut(new_entity) {
        *vis = Visibility::Visible;
    }

    party.active_index = slot_index;
    party.switch_cooldown.reset();
    sfx.write(PlaySfx { kind: SfxKind::PartySwap });
}
