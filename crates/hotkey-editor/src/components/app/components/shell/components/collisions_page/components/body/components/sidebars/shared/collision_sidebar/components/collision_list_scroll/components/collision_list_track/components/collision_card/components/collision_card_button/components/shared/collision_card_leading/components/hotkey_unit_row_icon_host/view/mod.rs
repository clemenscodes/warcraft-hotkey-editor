#[derive(Clone, PartialEq)]
pub struct HotkeyUnitRowIconHostView {
    pub icon_url: Option<String>,
    pub alt: String,
}

impl ddd::View for HotkeyUnitRowIconHostView {}
