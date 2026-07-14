#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum EditableKeycapState {
    #[default]
    Idle,
    Editing,
}
