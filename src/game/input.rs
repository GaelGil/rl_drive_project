// import macroquad library for keyboard input reading
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub shoot: bool,
    pub restart: bool,
}

pub fn read_input() -> InputState {
    InputState {
        // is_key_down = true while holding the key
        // is_key_pressed = true only on frame key is pressed
        move_left: is_key_down(KeyCode::Left) || is_key_down(KeyCode::A),
        move_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
        shoot: is_key_pressed(KeyCode::Space),
        restart: is_key_down(KeyCode::R),
    }
}
