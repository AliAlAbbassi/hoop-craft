use bevy::prelude::*;

/// Enemy marker with HP and stats.
#[derive(Component)]
pub struct Enemy {
    pub name: String,
    pub enemy_type: EnemyType,
    pub max_hp: f32,
    pub current_hp: f32,
    pub attack: f32,
    pub defense: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            name: "Slime".to_string(),
            enemy_type: EnemyType::Slime,
            max_hp: 500.0,
            current_hp: 500.0,
            attack: 30.0,
            defense: 20.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Slime,
    Hilichurl,
    Boss,
}

/// AI state machine state.
#[derive(Component, Default, PartialEq, Eq)]
pub enum AiState {
    #[default]
    Idle,
    Chase,
    Attack,
    Stagger,
}
