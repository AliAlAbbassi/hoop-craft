use avian3d::prelude::*;
use bevy::prelude::*;

use crate::audio_reactive::{ReactiveFloor, ReactiveNeon, ReactiveSpotlight};

/// Spawn a nightclub environment.
pub fn spawn_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ── Dance floor (reflective dark surface with subtle color) ──
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(20.0, 20.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.05, 0.08),
            metallic: 0.8,
            perceptual_roughness: 0.15,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        avian3d::prelude::Collider::half_space(Vec3::Y),
    ));

    // ── Glowing dance floor tiles (colored panels) ──
    let tile_colors = [
        Color::srgb(0.8, 0.1, 0.3),  // hot pink
        Color::srgb(0.1, 0.3, 0.9),  // blue
        Color::srgb(0.6, 0.1, 0.8),  // purple
        Color::srgb(0.1, 0.8, 0.6),  // cyan
        Color::srgb(0.9, 0.4, 0.1),  // orange
        Color::srgb(0.2, 0.8, 0.2),  // green
    ];

    for row in -3_i32..=3 {
        for col in -3_i32..=3 {
            let color_idx = ((row + col).unsigned_abs() as usize) % tile_colors.len();
            let emissive_color = tile_colors[color_idx];
            let r = emissive_color.to_srgba();

            let base_emissive = bevy::color::LinearRgba::new(r.red * 2.0, r.green * 2.0, r.blue * 2.0, 1.0);
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(2.4, 0.02, 2.4))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: emissive_color.with_alpha(0.8),
                    emissive: base_emissive,
                    metallic: 0.6,
                    perceptual_roughness: 0.2,
                    ..default()
                })),
                Transform::from_xyz(col as f32 * 2.6, 0.015, row as f32 * 2.6),
                ReactiveFloor { base_emissive },
            ));
        }
    }

    // ── Walls (dark with neon trim) ──
    let wall_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.06, 0.1),
        perceptual_roughness: 0.9,
        ..default()
    });

    // Back wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(42.0, 8.0, 0.3))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(0.0, 4.0, -20.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(42.0, 8.0, 0.3),
    ));
    // Front wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(42.0, 8.0, 0.3))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(0.0, 4.0, 20.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(42.0, 8.0, 0.3),
    ));
    // Left wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 8.0, 40.0))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(-20.0, 4.0, 0.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(0.3, 8.0, 40.0),
    ));
    // Right wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 8.0, 40.0))),
        MeshMaterial3d(wall_mat.clone()),
        Transform::from_xyz(20.0, 4.0, 0.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(0.3, 8.0, 40.0),
    ));

    // ── Ceiling ──
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(42.0, 0.3, 42.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.03, 0.02, 0.05),
            ..default()
        })),
        Transform::from_xyz(0.0, 8.0, 0.0),
    ));

    // ── Neon wall strips ──
    let neon_configs: [(Vec3, Vec3, Color); 8] = [
        // Back wall neons
        (Vec3::new(0.0, 6.5, -19.7), Vec3::new(30.0, 0.08, 0.08), Color::srgb(1.0, 0.0, 0.5)),
        (Vec3::new(0.0, 2.0, -19.7), Vec3::new(30.0, 0.08, 0.08), Color::srgb(0.0, 0.5, 1.0)),
        // Side wall neons
        (Vec3::new(-19.7, 5.0, 0.0), Vec3::new(0.08, 0.08, 30.0), Color::srgb(0.5, 0.0, 1.0)),
        (Vec3::new(19.7, 5.0, 0.0), Vec3::new(0.08, 0.08, 30.0), Color::srgb(0.0, 1.0, 0.5)),
        // Front wall neons
        (Vec3::new(0.0, 6.5, 19.7), Vec3::new(30.0, 0.08, 0.08), Color::srgb(1.0, 0.3, 0.0)),
        (Vec3::new(0.0, 2.0, 19.7), Vec3::new(30.0, 0.08, 0.08), Color::srgb(0.0, 1.0, 1.0)),
        // Extra accent strips
        (Vec3::new(-19.7, 3.5, 0.0), Vec3::new(0.08, 0.08, 30.0), Color::srgb(1.0, 0.0, 0.8)),
        (Vec3::new(19.7, 3.5, 0.0), Vec3::new(0.08, 0.08, 30.0), Color::srgb(0.3, 0.0, 1.0)),
    ];

    for (pos, size, color) in neon_configs {
        let c = color.to_srgba();
        let base_emissive = bevy::color::LinearRgba::new(c.red * 8.0, c.green * 8.0, c.blue * 8.0, 1.0);
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                emissive: base_emissive,
                ..default()
            })),
            Transform::from_translation(pos),
            ReactiveNeon { base_emissive },
        ));
    }

    // ── DJ booth (back center) ──
    let booth_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.08, 0.15),
        metallic: 0.5,
        perceptual_roughness: 0.4,
        ..default()
    });

    // DJ table
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 1.2, 1.5))),
        MeshMaterial3d(booth_mat.clone()),
        Transform::from_xyz(0.0, 0.6, -17.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(5.0, 1.2, 1.5),
    ));

    // DJ booth glow strip
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 0.1, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.8),
            emissive: bevy::color::LinearRgba::new(10.0, 0.0, 8.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.25, -16.2),
    ));

    // DJ speakers (two tall boxes)
    for x in [-4.0, 4.0] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.5, 3.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.08, 0.08, 0.08),
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(x, 1.5, -18.0),
            RigidBody::Static,
            avian3d::prelude::Collider::cuboid(1.5, 3.0, 1.0),
        ));

        // Speaker cone glow
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.4, 0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.0, 0.6),
                emissive: bevy::color::LinearRgba::new(3.0, 0.0, 6.0, 1.0),
                ..default()
            })),
            Transform::from_xyz(x, 2.0, -17.45)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ));
    }

    // ── Bar counter (side) ──
    let bar_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.08, 0.05),
        metallic: 0.3,
        perceptual_roughness: 0.5,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.1, 10.0))),
        MeshMaterial3d(bar_mat.clone()),
        Transform::from_xyz(17.0, 0.55, -5.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(1.2, 1.1, 10.0),
    ));

    // Bar top (shiny surface)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.4, 0.06, 10.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.12),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(17.0, 1.13, -5.0),
    ));

    // Bar underglow
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.05, 9.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.6, 1.0),
            emissive: bevy::color::LinearRgba::new(0.0, 4.0, 8.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(17.0, 0.03, -5.0),
    ));

    // ── Bar stools ──
    let stool_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.18),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });

    for z in [-9.0, -7.0, -5.0, -3.0, -1.0] {
        // Stool leg
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.04, 0.7))),
            MeshMaterial3d(stool_mat.clone()),
            Transform::from_xyz(15.8, 0.35, z),
        ));
        // Stool seat
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.2, 0.06))),
            MeshMaterial3d(stool_mat.clone()),
            Transform::from_xyz(15.8, 0.73, z),
        ));
    }

    // ── VIP lounge area (opposite side) ──
    let couch_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.05, 0.12),
        perceptual_roughness: 0.7,
        ..default()
    });

    // L-shaped couch - back section
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5.0, 0.7, 1.2))),
        MeshMaterial3d(couch_mat.clone()),
        Transform::from_xyz(-16.0, 0.35, -8.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(5.0, 0.7, 1.2),
    ));
    // L-shaped couch - side section
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 0.7, 4.0))),
        MeshMaterial3d(couch_mat.clone()),
        Transform::from_xyz(-18.0, 0.35, -5.5),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(1.2, 0.7, 4.0),
    ));
    // VIP table
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.12),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(-15.5, 0.25, -5.5),
    ));

    // ── Disco ball (center ceiling) ──
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.85),
            metallic: 1.0,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_xyz(0.0, 7.0, 0.0),
    ));

    // ── Spotlight columns on dance floor ──
    let column_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.06, 0.06, 0.08),
        metallic: 0.7,
        perceptual_roughness: 0.3,
        ..default()
    });

    let column_positions = [
        Vec3::new(-8.0, 0.0, -8.0),
        Vec3::new(8.0, 0.0, -8.0),
        Vec3::new(-8.0, 0.0, 8.0),
        Vec3::new(8.0, 0.0, 8.0),
    ];

    let spot_colors = [
        Color::srgb(1.0, 0.0, 0.4),
        Color::srgb(0.0, 0.4, 1.0),
        Color::srgb(0.4, 0.0, 1.0),
        Color::srgb(0.0, 1.0, 0.4),
    ];

    for (i, pos) in column_positions.iter().enumerate() {
        // Column
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.15, 8.0))),
            MeshMaterial3d(column_mat.clone()),
            Transform::from_xyz(pos.x, 4.0, pos.z),
            RigidBody::Static,
            avian3d::prelude::Collider::cylinder(0.15, 8.0),
        ));

        // Spotlight at top
        let sc = spot_colors[i].to_srgba();
        commands.spawn((
            PointLight {
                color: spot_colors[i],
                intensity: 50_000.0,
                range: 20.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(pos.x, 7.5, pos.z),
            ReactiveSpotlight { base_intensity: 50_000.0 },
        ));

        // Light ring at base
        commands.spawn((
            Mesh3d(meshes.add(Torus::new(0.2, 0.25))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: spot_colors[i],
                emissive: bevy::color::LinearRgba::new(sc.red * 5.0, sc.green * 5.0, sc.blue * 5.0, 1.0),
                ..default()
            })),
            Transform::from_xyz(pos.x, 0.05, pos.z),
        ));
    }

    // ── Center dance floor spotlight (from above) ──
    commands.spawn((
        PointLight {
            color: Color::srgb(0.9, 0.5, 1.0),
            intensity: 80_000.0,
            range: 25.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 7.0, 0.0),
        ReactiveSpotlight { base_intensity: 80_000.0 },
    ));

    // ── Raised platform / stage area ──
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.4, 4.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.06, 0.1),
            metallic: 0.5,
            perceptual_roughness: 0.3,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.2, -14.0),
        RigidBody::Static,
        avian3d::prelude::Collider::cuboid(8.0, 0.4, 4.0),
    ));

    // Stage edge glow
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.08, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.5),
            emissive: bevy::color::LinearRgba::new(10.0, 0.0, 5.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.42, -12.0),
    ));
}
