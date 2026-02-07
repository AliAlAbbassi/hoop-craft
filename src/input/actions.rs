use bevy::prelude::*;

/// Processed input actions for the game.
#[derive(Resource, Default)]
pub struct InputActions {
    /// Movement direction (normalized XZ plane).
    pub movement: Vec2,
    /// Camera rotation delta from mouse.
    pub look_delta: Vec2,
    /// Jump pressed this frame.
    pub jump: bool,
    /// Sprint held.
    pub sprint: bool,
    /// Normal attack pressed.
    pub attack: bool,
    /// Skill (E) pressed.
    pub skill: bool,
    /// Burst (Q) pressed.
    pub burst: bool,
    /// Party slot selection (1-4), None if no key pressed.
    pub party_slot: Option<usize>,
    /// Scroll wheel delta for camera zoom.
    pub zoom_delta: f32,
}

pub fn update_input_actions(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<bevy::input::mouse::MouseMotion>,
    mut scroll: MessageReader<bevy::input::mouse::MouseWheel>,
    mut actions: ResMut<InputActions>,
) {
    // Movement (WASD)
    let mut move_dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        move_dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        move_dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        move_dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        move_dir.x += 1.0;
    }
    actions.movement = if move_dir.length_squared() > 0.0 {
        move_dir.normalize()
    } else {
        Vec2::ZERO
    };

    // Camera look (mouse motion, only when right-click held)
    let mut look = Vec2::ZERO;
    for ev in mouse_motion.read() {
        if mouse_buttons.pressed(MouseButton::Right) {
            look += ev.delta;
        }
    }
    actions.look_delta = look;

    // Zoom
    let mut zoom = 0.0;
    for ev in scroll.read() {
        zoom += ev.y;
    }
    actions.zoom_delta = zoom;

    // Actions
    actions.jump = keyboard.just_pressed(KeyCode::Space);
    actions.sprint = keyboard.pressed(KeyCode::ShiftLeft);
    actions.attack = mouse_buttons.just_pressed(MouseButton::Left);
    actions.skill = keyboard.just_pressed(KeyCode::KeyE);
    actions.burst = keyboard.just_pressed(KeyCode::KeyQ);

    // Party switching (1-4)
    actions.party_slot = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else {
        None
    };
}
