use super::state::EditableKeycapState;

#[derive(Clone, PartialEq)]
pub struct EditableKeycapView {
    pub label: String,
    pub state: EditableKeycapState,
}

impl ddd::View for EditableKeycapView {}
