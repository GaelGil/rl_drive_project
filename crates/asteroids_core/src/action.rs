pub enum Action {
    // Actions available to agent
    NoOp,
    MoveLeft,
    MoveRight,
    Shoot,
}

impl Action {
    pub fn from_index(index: usize) -> Self {
        // convert an index into a game action
        // returns action to take
        match index {
            1 => Self::MoveLeft,
            2 => Self::MoveRight,
            3 => Self::Shoot,
            _ => Self::NoOp,
        }
    }

    pub fn action_count() -> usize {
        // number of actions available
        return 4;
    }
}
