use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_vrm1::prelude::*;

use crate::combat::components::{ComboState, Skill, Burst};
use crate::elements::types::Element;
use crate::party::components::Party;
use super::components::*;

/// Message to spawn a new character.
#[derive(Message)]
pub struct SpawnCharacterEvent {
    pub name: String,
    pub element: Element,
    pub vrm_path: Option<String>,
    pub position: Vec3,
    pub make_active: bool,
}

/// Spawn the default party on game start.
pub fn spawn_default_character(mut writer: MessageWriter<SpawnCharacterEvent>) {
    // Main character - Pyro
    writer.write(SpawnCharacterEvent {
        name: "Anya".to_string(),
        element: Element::Pyro,
        vrm_path: Some("models/characters/blonde_girl.vrm".to_string()),
        position: Vec3::new(0.0, 0.8, 0.0),
        make_active: true,
    });

    // Second party member - Hydro
    writer.write(SpawnCharacterEvent {
        name: "Mira".to_string(),
        element: Element::Hydro,
        vrm_path: Some("models/characters/char_b.vrm".to_string()),
        position: Vec3::new(0.0, 0.8, 0.0),
        make_active: false,
    });

    // Third party member - Electro
    writer.write(SpawnCharacterEvent {
        name: "Kai".to_string(),
        element: Element::Electro,
        vrm_path: Some("models/characters/char_c.vrm".to_string()),
        position: Vec3::new(0.0, 0.8, 0.0),
        make_active: false,
    });

    // Fourth party member - Cryo
    writer.write(SpawnCharacterEvent {
        name: "Suki".to_string(),
        element: Element::Cryo,
        vrm_path: Some("models/characters/char_d.vrm".to_string()),
        position: Vec3::new(0.0, 0.8, 0.0),
        make_active: false,
    });
}

/// Handle character spawn events.
pub fn handle_spawn_events(
    mut commands: Commands,
    mut reader: MessageReader<SpawnCharacterEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut party: ResMut<Party>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        let entity = if let Some(vrm_path) = &event.vrm_path {
            // Spawn VRM character
            commands
                .spawn((
                    VrmHandle(asset_server.load(vrm_path)),
                    Transform::from_translation(event.position),
                ))
                .id()
        } else {
            // Spawn placeholder capsule character
            let capsule_color = event.element.color();
            commands
                .spawn((
                    Mesh3d(meshes.add(Capsule3d::new(0.3, 1.0))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: capsule_color,
                        ..default()
                    })),
                    Transform::from_translation(event.position),
                ))
                .id()
        };

        // Attach character components
        commands.entity(entity).insert((
            Character {
                name: event.name.clone(),
                element: event.element,
            },
            CharacterStats::default(),
            AnimationState::default(),
            CharacterVelocity::default(),
            ComboState::default(),
            Skill {
                cooldown: Timer::from_seconds(6.0, TimerMode::Once),
                damage_multiplier: 3.0,
                element: event.element,
                gauge_units: 1.0,
            },
            Burst {
                energy_cost: 60.0,
                damage_multiplier: 8.0,
                element: event.element,
                gauge_units: 2.0,
            },
            // Physics (kinematic - we handle movement manually)
            RigidBody::Kinematic,
            avian3d::prelude::Collider::capsule(0.3, 1.0),
        ));

        if event.make_active {
            commands.entity(entity).insert(ActiveCharacter);
        }

        // Non-active party members start hidden
        if !event.make_active {
            commands.entity(entity).insert(Visibility::Hidden);
        }

        // Register in party
        party.add_character(entity);
    }
}
