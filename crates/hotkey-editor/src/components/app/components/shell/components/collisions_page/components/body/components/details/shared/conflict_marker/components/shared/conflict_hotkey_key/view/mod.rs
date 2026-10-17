#[derive(Clone, PartialEq)]
pub struct ConflictHotkeyKeyView {
    pub text: String,
}

impl ddd::Layered for ConflictHotkeyKeyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictHotkeyKeyView {}
