use bevy::prelude::*;

use super::components::ActiveCharacter;

/// Marker for the weapon entity.
#[derive(Component)]
pub struct Weapon;

/// Spawn an AK47-style weapon built from primitives, attached to the active character.
/// Uses a Local bool to only run once.
pub fn spawn_weapon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<Entity, With<ActiveCharacter>>,
    mut spawned: Local<bool>,
) {
    if *spawned {
        return;
    }
    let Ok(char_entity) = query.single() else {
        return;
    };
    *spawned = true;

    let metal_dark = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18),
        metallic: 0.9,
        perceptual_roughness: 0.3,
        ..default()
    });
    let metal_barrel = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.12),
        metallic: 0.95,
        perceptual_roughness: 0.2,
        ..default()
    });
    let wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.45, 0.25, 0.1),
        metallic: 0.0,
        perceptual_roughness: 0.8,
        ..default()
    });

    // Weapon root - positioned at right hand area, angled forward
    let weapon = commands
        .spawn((
            Weapon,
            Transform::from_xyz(0.35, 0.5, 0.2)
                .with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, -0.3, -0.15)),
            Visibility::Inherited,
        ))
        .with_children(|gun| {
            // ── Barrel (long thin cylinder) ──
            gun.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.015, 0.5))),
                MeshMaterial3d(metal_barrel.clone()),
                Transform::from_xyz(0.0, 0.0, 0.45)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ));

            // ── Gas tube (thinner cylinder above barrel) ──
            gun.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.01, 0.35))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, 0.02, 0.38)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ));

            // ── Receiver / main body ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.045, 0.055, 0.22))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, -0.01, 0.1),
            ));

            // ── Handguard (wood) ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.04, 0.04, 0.15))),
                MeshMaterial3d(wood.clone()),
                Transform::from_xyz(0.0, -0.01, 0.3),
            ));

            // ── Stock (wood, angled slightly down) ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.035, 0.06, 0.2))),
                MeshMaterial3d(wood.clone()),
                Transform::from_xyz(0.0, -0.02, -0.1)
                    .with_rotation(Quat::from_rotation_x(0.05)),
            ));

            // ── Stock butt plate ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.035, 0.07, 0.015))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, -0.025, -0.2),
            ));

            // ── Magazine (curved box, angled forward) ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.025, 0.1, 0.04))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, -0.08, 0.08)
                    .with_rotation(Quat::from_rotation_x(0.15)),
            ));

            // ── Trigger guard ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.01, 0.03, 0.04))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, -0.04, 0.02),
            ));

            // ── Grip (pistol grip) ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.025, 0.06, 0.03))),
                MeshMaterial3d(wood.clone()),
                Transform::from_xyz(0.0, -0.06, -0.01)
                    .with_rotation(Quat::from_rotation_x(-0.25)),
            ));

            // ── Front sight post ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.005, 0.02, 0.005))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, 0.025, 0.6),
            ));

            // ── Rear sight ──
            gun.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.02, 0.015, 0.005))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, 0.03, 0.15),
            ));

            // ── Muzzle brake ──
            gun.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.02, 0.03))),
                MeshMaterial3d(metal_dark.clone()),
                Transform::from_xyz(0.0, 0.0, 0.71)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ));
        })
        .id();

    // Attach weapon as child of active character
    commands.entity(char_entity).add_child(weapon);
}

/// Keep the weapon attached to whichever character is active.
/// When party swaps happen, re-parent the weapon.
pub fn follow_active_character(
    mut commands: Commands,
    weapon_query: Query<(Entity, &ChildOf), With<Weapon>>,
    active_query: Query<Entity, With<ActiveCharacter>>,
) {
    let Ok(active) = active_query.single() else {
        return;
    };
    let Ok((weapon_entity, child_of)) = weapon_query.single() else {
        return;
    };

    if child_of.parent() != active {
        commands.entity(active).add_child(weapon_entity);
    }
}
