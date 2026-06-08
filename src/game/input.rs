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
        move_left: is_key_down(KeyCode::left) || is_key_down(KeyCode::A),
        move_right: is_key_down(KeyCode::Right) || is_key_down(KeyCode::D),
        shoot: is_key_pressed(KeyCode::Space),
        restart: is_key_down(KeyCode::R),
    }
}
