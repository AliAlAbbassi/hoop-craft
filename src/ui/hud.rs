use bevy::prelude::*;

use super::hp_bar;
use super::action_bar;
use super::party_ui;

/// Root HUD entity marker.
#[derive(Component)]
pub struct HudRoot;

/// Marker for the active character name display.
#[derive(Component)]
pub struct CharacterNameDisplay;

/// Spawn the full HUD layout.
pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            // ── Top section: character info + HP bars ──
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    padding: UiRect::all(Val::Px(20.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                })
                .with_children(|top| {
                    // Character name
                    top.spawn((
                        CharacterNameDisplay,
                        Text::new("Anya"),
                        TextFont {
                            font_size: 22.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // HP + stamina bars
                    hp_bar::build_hp_section(top);
                });

            // ── Center: crosshair dot ──
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|center| {
                    center.spawn((
                        Node {
                            width: Val::Px(4.0),
                            height: Val::Px(4.0),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                    ));
                });

            // ── Bottom section: party portraits (left) + action bar (right) ──
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::End,
                    padding: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(0.0), Val::Px(20.0)),
                    ..default()
                })
                .with_children(|bottom| {
                    party_ui::build_party_portraits(bottom);
                    action_bar::build_action_bar(bottom);
                });
        });
}

/// Update the character name display when active character changes.
pub fn update_character_name(
    active_query: Query<&crate::character::components::Character, With<crate::character::components::ActiveCharacter>>,
    mut name_query: Query<&mut Text, With<CharacterNameDisplay>>,
) {
    if let Ok(character) = active_query.single() {
        if let Ok(mut text) = name_query.single_mut() {
            **text = character.name.clone();
        }
    }
}
