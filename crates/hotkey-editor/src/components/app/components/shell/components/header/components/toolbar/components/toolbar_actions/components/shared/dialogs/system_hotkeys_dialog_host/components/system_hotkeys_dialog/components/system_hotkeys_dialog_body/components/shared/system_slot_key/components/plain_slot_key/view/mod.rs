/// The published `View` contract mirroring [`PlainSlotKeyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlainSlotKeyView {
    pub label: String,
}

impl ddd::View for PlainSlotKeyView {}
