/// The published `View` contract mirroring [`TopHotkeyMarkerProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TopHotkeyMarkerView {
    pub label: String,
}

impl ddd::View for TopHotkeyMarkerView {}
