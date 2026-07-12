/// The published `View` contract mirroring [`SystemSlotKeyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemSlotKeyView {
    pub label: String,
    pub conflict: bool,
}

impl ddd::View for SystemSlotKeyView {}
