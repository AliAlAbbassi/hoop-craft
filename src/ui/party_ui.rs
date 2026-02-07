use bevy::prelude::*;

use crate::party::components::Party;
use crate::character::components::Character;

/// Marker for a party slot UI element.
#[derive(Component)]
pub struct PartySlotUi(pub usize);

/// Marker for the element color indicator in a party slot.
#[derive(Component)]
pub struct PartySlotElement(pub usize);

pub fn build_party_portraits(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            column_gap: Val::Px(8.0),
            align_items: AlignItems::End,
            ..default()
        })
        .with_children(|row| {
            for i in 0..4 {
                let key_label = format!("{}", i + 1);
                row.spawn((
                    PartySlotUi(i),
                    Node {
                        width: Val::Px(52.0),
                        height: Val::Px(52.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.25, 0.7)),
                ))
                .with_children(|slot| {
                    // Element color bar at top
                    slot.spawn((
                        PartySlotElement(i),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(3.0),
                            border_radius: BorderRadius::all(Val::Px(1.5)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.4)),
                    ));

                    // Key number
                    slot.spawn((
                        Text::new(key_label),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                    ));
                });
            }
        });
}

pub fn update_party_portraits(
    party: Res<Party>,
    char_query: Query<&Character>,
    mut slot_query: Query<(&PartySlotUi, &mut BackgroundColor), Without<PartySlotElement>>,
    mut element_query: Query<(&PartySlotElement, &mut BackgroundColor), Without<PartySlotUi>>,
) {
    for (slot, mut bg) in &mut slot_query {
        if slot.0 == party.active_index && party.slots[slot.0].is_some() {
            *bg = BackgroundColor(Color::srgba(0.3, 0.5, 0.9, 0.85));
        } else if party.slots[slot.0].is_some() {
            *bg = BackgroundColor(Color::srgba(0.15, 0.15, 0.25, 0.7));
        } else {
            *bg = BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.3));
        }
    }

    for (elem_slot, mut bg) in &mut element_query {
        if let Some(entity) = party.slots[elem_slot.0] {
            if let Ok(character) = char_query.get(entity) {
                *bg = BackgroundColor(character.element.color());
            }
        }
    }
}
