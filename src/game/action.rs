pub enum Action {
    // comment
    NoOp,
    MoveLeft,
    MoveRight,
    Shoot,
}

impl Action {
    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::MoveLeft,
            2 => Self::MoveRight,
            3 => Self::Shoot,
            _ => Self::NoOp,
        }
    }

    pub fn action_count() -> usize {
        return 4;
    }
}
