#[derive(Clone, PartialEq)]
pub struct PlainSlotKeyView {
    pub label: String,
}

impl ddd::Layered for PlainSlotKeyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PlainSlotKeyView {}
