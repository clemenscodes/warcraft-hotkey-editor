#[derive(Clone, PartialEq)]
pub struct ConflictSlotKeyView {
    pub label: String,
}

impl ddd::Layered for ConflictSlotKeyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictSlotKeyView {}
