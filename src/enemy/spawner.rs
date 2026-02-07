use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_vrm1::prelude::*;

use crate::combat::components::HurtBox;
use crate::elements::types::Element;

use super::components::*;

/// Spawn enemies using VRM character models.
pub fn spawn_test_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // VRM models to use for enemies (cycle through available models)
    let enemy_models = [
        "models/characters/blonde_girl.vrm",
        "models/characters/char_b.vrm",
        "models/characters/char_c.vrm",
        "models/characters/char_d.vrm",
    ];

    let enemy_configs: Vec<(&str, Vec3, Element, EnemyType, f32, f32, f32, f32, f32)> = vec![
        // (name, pos, element, type, hp, atk, def, scale, model_index mapped below)
        ("Hydro Agent",    Vec3::new(5.0, 0.8, 5.0),   Element::Hydro,   EnemyType::Hilichurl, 800.0, 45.0, 30.0, 1.0, 0.0),
        ("Pyro Agent",     Vec3::new(-4.0, 0.8, 6.0),  Element::Pyro,    EnemyType::Hilichurl, 700.0, 50.0, 25.0, 0.9, 1.0),
        ("Electro Scout",  Vec3::new(7.0, 0.8, -3.0),  Element::Electro, EnemyType::Slime,     500.0, 35.0, 20.0, 0.85, 2.0),
        ("Cryo Enforcer",  Vec3::new(-6.0, 0.8, -5.0), Element::Cryo,    EnemyType::Hilichurl, 900.0, 40.0, 35.0, 1.1, 3.0),
        ("Dendro Lurker",  Vec3::new(3.0, 0.8, 8.0),   Element::Dendro,  EnemyType::Slime,     400.0, 30.0, 15.0, 0.8, 0.0),
        ("Pyro Captain",   Vec3::new(3.0, 0.8, -7.0),  Element::Pyro,    EnemyType::Boss,      1500.0, 60.0, 40.0, 1.3, 1.0),
        ("Geo Sentinel",   Vec3::new(-8.0, 0.8, -2.0), Element::Geo,     EnemyType::Boss,      1200.0, 55.0, 50.0, 1.2, 2.0),
    ];

    for (i, (name, pos, _element, enemy_type, hp, atk, def, scale, _)) in enemy_configs.iter().enumerate() {
        let model_path = enemy_models[i % enemy_models.len()];

        commands.spawn((
            VrmHandle(asset_server.load(model_path)),
            Transform::from_translation(*pos)
                .with_scale(Vec3::splat(*scale)),
            Enemy {
                name: name.to_string(),
                enemy_type: *enemy_type,
                max_hp: *hp,
                current_hp: *hp,
                attack: *atk,
                defense: *def,
            },
            AiState::default(),
            HurtBox,
            RigidBody::Dynamic,
            avian3d::prelude::Collider::capsule(0.3 * scale, 1.0 * scale),
            LockedAxes::ROTATION_LOCKED,
        ));
    }
}
