/// The published `View` contract mirroring [`ConflictHotkeyKeyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictHotkeyKeyView {
    pub text: String,
}

impl ddd::View for ConflictHotkeyKeyView {}
