/// The published `View` contract mirroring [`PlainSlotKeyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PlainSlotKeyView {
    pub label: String,
}

impl ddd::View for PlainSlotKeyView {}
