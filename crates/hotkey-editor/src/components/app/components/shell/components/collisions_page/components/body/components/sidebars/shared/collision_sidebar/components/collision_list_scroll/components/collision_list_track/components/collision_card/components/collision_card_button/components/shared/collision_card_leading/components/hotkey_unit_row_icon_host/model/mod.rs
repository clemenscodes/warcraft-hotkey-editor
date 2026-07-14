use super::view::HotkeyUnitRowIconHostView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitRowIconHostModel {
    pub icon_url: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&HotkeyUnitRowIconHostView> for HotkeyUnitRowIconHostModel {
    fn from(view: &HotkeyUnitRowIconHostView) -> Self {
        let HotkeyUnitRowIconHostView { icon_url, alt } = view.clone();
        Self { icon_url, alt }
    }
}

impl ddd::Model for HotkeyUnitRowIconHostModel {
    type View = HotkeyUnitRowIconHostView;
}
