/// The published `View` contract mirroring [`TopHotkeyMarkerModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TopHotkeyMarkerView {
    pub label: String,
}

impl ddd::View for TopHotkeyMarkerView {}
