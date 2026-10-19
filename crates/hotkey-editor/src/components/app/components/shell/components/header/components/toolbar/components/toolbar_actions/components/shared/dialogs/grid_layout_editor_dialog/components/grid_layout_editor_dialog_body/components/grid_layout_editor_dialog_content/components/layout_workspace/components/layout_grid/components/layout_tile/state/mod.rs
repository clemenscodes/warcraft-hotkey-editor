use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum LayoutTileState {
    #[default]
    Idle,
    Editing,
}

impl From<LayoutTileState> for EditableKeycapState {
    fn from(state: LayoutTileState) -> Self {
        match state {
            LayoutTileState::Idle => Self::Idle,
            LayoutTileState::Editing => Self::Editing,
        }
    }
}
