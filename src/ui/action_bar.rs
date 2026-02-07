use bevy::prelude::*;

/// Marker for skill icon.
#[derive(Component)]
pub struct SkillIcon;

/// Marker for burst icon.
#[derive(Component)]
pub struct BurstIcon;

pub fn build_action_bar(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn(Node {
            column_gap: Val::Px(8.0),
            align_items: AlignItems::End,
            ..default()
        })
        .with_children(|bar| {
            // Skill (E) button
            spawn_ability_icon(bar, "E", Color::srgb(0.3, 0.6, 0.9), SkillIcon);
            // Burst (Q) button
            spawn_ability_icon(bar, "Q", Color::srgb(0.9, 0.7, 0.2), BurstIcon);
        });
}

fn spawn_ability_icon<M: Component>(parent: &mut ChildSpawnerCommands, label: &str, color: Color, marker: M) {
    parent
        .spawn((
            marker,
            Node {
                width: Val::Px(56.0),
                height: Val::Px(56.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(color.with_alpha(0.7)),
        ))
        .with_children(|icon| {
            icon.spawn((
                Text::new(label.to_string()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn update_action_bar(
    // Future: update cooldown overlays based on skill/burst timers
) {
}
