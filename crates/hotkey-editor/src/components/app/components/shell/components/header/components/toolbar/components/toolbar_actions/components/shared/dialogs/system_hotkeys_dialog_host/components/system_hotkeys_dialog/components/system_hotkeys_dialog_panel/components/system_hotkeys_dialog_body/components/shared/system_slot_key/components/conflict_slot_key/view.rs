/// The published `View` contract mirroring [`ConflictSlotKeyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictSlotKeyView {
    pub label: String,
}

impl ddd::View for ConflictSlotKeyView {}
