/// The published `View` contract mirroring [`HotkeyUnitRowIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HotkeyUnitRowIconView {
    pub icon_url: Option<String>,
    pub alt: String,
}

impl ddd::View for HotkeyUnitRowIconView {}
