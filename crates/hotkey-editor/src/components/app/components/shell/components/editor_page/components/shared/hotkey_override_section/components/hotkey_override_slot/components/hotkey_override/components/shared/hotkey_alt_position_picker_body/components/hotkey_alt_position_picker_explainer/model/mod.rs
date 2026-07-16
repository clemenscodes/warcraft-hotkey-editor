use super::view::HotkeyAltPositionPickerExplainerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyAltPositionPickerExplainerModel {
    #[props(into)]
    pub text: String,
}

impl From<&HotkeyAltPositionPickerExplainerView> for HotkeyAltPositionPickerExplainerModel {
    fn from(view: &HotkeyAltPositionPickerExplainerView) -> Self {
        let HotkeyAltPositionPickerExplainerView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for HotkeyAltPositionPickerExplainerModel {
    type View = HotkeyAltPositionPickerExplainerView;
}
