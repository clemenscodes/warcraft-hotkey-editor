#[derive(Clone, PartialEq)]
pub struct SystemSlotKeyView {
    pub label: String,
    pub conflict: bool,
}

impl ddd::View for SystemSlotKeyView {}
