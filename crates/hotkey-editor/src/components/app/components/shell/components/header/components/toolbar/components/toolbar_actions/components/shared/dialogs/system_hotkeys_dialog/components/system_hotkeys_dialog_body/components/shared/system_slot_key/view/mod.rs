#[derive(Clone, PartialEq)]
pub struct SystemSlotKeyView {
    pub label: String,
    pub conflict: bool,
}

impl ddd::Layered for SystemSlotKeyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SystemSlotKeyView {}
