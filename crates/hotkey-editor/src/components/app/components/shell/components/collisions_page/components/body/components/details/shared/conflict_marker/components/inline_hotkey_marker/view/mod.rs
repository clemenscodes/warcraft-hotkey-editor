/// The published `View` contract mirroring [`InlineHotkeyMarkerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InlineHotkeyMarkerView {
    pub label: String,
}

impl ddd::View for InlineHotkeyMarkerView {}
