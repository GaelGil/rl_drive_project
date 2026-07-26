#[derive(Debug, Clone, Copy, Default)]
pub struct InputState {
    pub move_left: bool,
    pub move_right: bool,
    pub shoot: bool,
    pub restart: bool,
}
