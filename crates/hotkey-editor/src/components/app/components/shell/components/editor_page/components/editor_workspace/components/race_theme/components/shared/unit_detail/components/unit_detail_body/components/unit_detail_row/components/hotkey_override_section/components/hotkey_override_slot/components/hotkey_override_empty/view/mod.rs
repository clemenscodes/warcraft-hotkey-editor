/// The published `View` contract mirroring [`HotkeyOverrideEmptyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideEmptyView {
    pub message: String,
}

impl ddd::View for HotkeyOverrideEmptyView {}
