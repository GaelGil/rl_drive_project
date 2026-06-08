#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    NoOp,
    MoveLeft,
    MoveRight,
    Shoot,
}
