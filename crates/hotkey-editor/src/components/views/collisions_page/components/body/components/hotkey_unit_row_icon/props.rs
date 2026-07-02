use dioxus::prelude::*;
/// A unit's portrait on a hotkey/position collision card.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyUnitRowIconProps {
    #[props(into)]
    pub src: String,
    #[props(into)]
    pub alt: String,
}
