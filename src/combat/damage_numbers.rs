use bevy::prelude::*;

use super::damage::DamageResolvedEvent;

/// Marker for floating damage number entities.
#[derive(Component)]
pub struct DamageNumber {
    pub lifetime: Timer,
    pub velocity: Vec3,
}

/// Spawn floating damage numbers from resolved damage events.
pub fn spawn_damage_numbers(
    mut commands: Commands,
    mut reader: MessageReader<DamageResolvedEvent>,
) {
    for event in reader.read() {
        let color = if event.is_crit {
            Color::srgb(1.0, 0.9, 0.0) // Gold for crits
        } else if let Some(element) = event.element {
            element.color()
        } else {
            Color::WHITE
        };

        let font_size = if event.is_crit { 32.0 } else { 24.0 };

        commands.spawn((
            DamageNumber {
                lifetime: Timer::from_seconds(1.2, TimerMode::Once),
                velocity: Vec3::new(
                    (rand::random::<f32>() - 0.5) * 1.0,
                    2.0,
                    0.0,
                ),
            },
            Text::new(format!("{:.0}", event.amount)),
            TextFont {
                font_size,
                ..default()
            },
            TextColor(color),
            Node {
                position_type: PositionType::Absolute,
                ..default()
            },
            DamageNumberWorldPos(event.position),
        ));
    }
}

/// Stores the world-space origin of a damage number for screen projection.
#[derive(Component)]
pub struct DamageNumberWorldPos(pub Vec3);

/// Animate damage numbers: float up, fade out, despawn.
pub fn animate_damage_numbers(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut DamageNumber,
        &mut DamageNumberWorldPos,
        &mut Node,
        &mut TextColor,
    )>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    time: Res<Time>,
) {
    let Ok((camera, cam_gtf)) = camera_q.single() else {
        return;
    };

    for (entity, mut dmg_num, mut world_pos, mut node, mut text_color) in &mut query {
        dmg_num.lifetime.tick(time.delta());

        // Move world position upward
        world_pos.0 += dmg_num.velocity * time.delta_secs();
        dmg_num.velocity.y -= 3.0 * time.delta_secs();

        // Project to screen space
        if let Ok(screen_pos) = camera.world_to_viewport(cam_gtf, world_pos.0) {
            node.left = Val::Px(screen_pos.x);
            node.top = Val::Px(screen_pos.y);
        }

        // Fade out
        let alpha = 1.0 - dmg_num.lifetime.fraction();
        text_color.0 = text_color.0.with_alpha(alpha);

        if dmg_num.lifetime.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
