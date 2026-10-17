#[derive(Clone, PartialEq)]
pub struct HotkeyUnitRowIconHostView {
    pub icon_url: Option<String>,
    pub alt: String,
}

impl ddd::Layered for HotkeyUnitRowIconHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HotkeyUnitRowIconHostView {}
