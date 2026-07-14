#[derive(Clone, PartialEq)]
pub struct ConflictSlotKeyView {
    pub label: String,
}

impl ddd::View for ConflictSlotKeyView {}
