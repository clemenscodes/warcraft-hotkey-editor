#[derive(Clone, PartialEq)]
pub struct HotkeyUnitRowIconView {
    pub icon_url: Option<String>,
    pub alt: String,
}

impl ddd::View for HotkeyUnitRowIconView {}
