use dioxus::prelude::*;
/// A unit's portrait on a hotkey/position collision card.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitRowIconProps {
    pub icon_url: Option<String>,
    #[props(into)]
    pub alt: String,
}
