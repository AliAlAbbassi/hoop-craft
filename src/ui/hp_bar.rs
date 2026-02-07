use bevy::prelude::*;

use crate::character::components::{ActiveCharacter, CharacterStats};
use crate::stamina::systems::Stamina;

/// Marker for the player HP bar fill.
#[derive(Component)]
pub struct PlayerHpBar;

/// Marker for the stamina bar fill.
#[derive(Component)]
pub struct StaminaBar;

pub fn build_hp_section(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            padding: UiRect::all(Val::Px(16.0)),
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|section| {
            // HP bar container
            section
                .spawn((
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(20.0),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ))
                .with_children(|bar_bg| {
                    bar_bg.spawn((
                        PlayerHpBar,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border_radius: BorderRadius::all(Val::Px(4.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.8, 0.3)),
                    ));
                });

            // Stamina bar container
            section
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(8.0),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)),
                ))
                .with_children(|bar_bg| {
                    bar_bg.spawn((
                        StaminaBar,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.8, 0.2)),
                    ));
                });
        });
}

pub fn update_hp_bars(
    stats_query: Query<&CharacterStats, With<ActiveCharacter>>,
    stamina: Option<Res<Stamina>>,
    mut hp_bar: Query<&mut Node, (With<PlayerHpBar>, Without<StaminaBar>)>,
    mut stam_bar: Query<&mut Node, (With<StaminaBar>, Without<PlayerHpBar>)>,
) {
    if let Ok(stats) = stats_query.single() {
        let hp_pct = (stats.current_hp / stats.max_hp * 100.0).clamp(0.0, 100.0);
        if let Ok(mut node) = hp_bar.single_mut() {
            node.width = Val::Percent(hp_pct);
        }
    }

    if let Some(stamina) = stamina {
        let stam_pct = (stamina.current / stamina.max * 100.0).clamp(0.0, 100.0);
        if let Ok(mut node) = stam_bar.single_mut() {
            node.width = Val::Percent(stam_pct);
        }
    }
}
