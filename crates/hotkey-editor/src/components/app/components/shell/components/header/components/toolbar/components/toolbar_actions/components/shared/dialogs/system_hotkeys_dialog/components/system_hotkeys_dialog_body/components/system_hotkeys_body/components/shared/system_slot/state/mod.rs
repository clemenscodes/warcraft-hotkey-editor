#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SystemSlotState {
    #[default]
    Idle,
    Highlighted,
    Conflict,
}
