#[derive(Clone, PartialEq)]
pub struct PlainSlotKeyView {
    pub label: String,
}

impl ddd::View for PlainSlotKeyView {}
