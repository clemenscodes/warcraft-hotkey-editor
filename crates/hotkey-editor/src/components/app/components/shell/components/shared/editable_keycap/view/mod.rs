use super::state::EditableKeycapState;

#[derive(Clone, PartialEq)]
pub struct EditableKeycapView {
    pub label: String,
    pub state: EditableKeycapState,
}

impl ddd::Layered for EditableKeycapView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for EditableKeycapView {}
