use super::view::HotkeyUnitRowIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitRowIconModel {
    pub icon_url: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&HotkeyUnitRowIconView> for HotkeyUnitRowIconModel {
    fn from(view: &HotkeyUnitRowIconView) -> Self {
        let HotkeyUnitRowIconView { icon_url, alt } = view.clone();
        Self { icon_url, alt }
    }
}

impl ddd::Model for HotkeyUnitRowIconModel {
    type View = HotkeyUnitRowIconView;
}
