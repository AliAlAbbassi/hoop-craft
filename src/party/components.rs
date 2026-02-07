use bevy::prelude::*;

/// Party resource: up to 4 character entity slots.
#[derive(Resource, Default)]
pub struct Party {
    pub slots: [Option<Entity>; 4],
    pub active_index: usize,
    pub switch_cooldown: Timer,
}

impl Party {
    pub fn new() -> Self {
        Self {
            slots: [None; 4],
            active_index: 0,
            switch_cooldown: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }

    pub fn active_entity(&self) -> Option<Entity> {
        self.slots[self.active_index]
    }

    pub fn add_character(&mut self, entity: Entity) -> Option<usize> {
        for (i, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(entity);
                return Some(i);
            }
        }
        None // Party full
    }
}
