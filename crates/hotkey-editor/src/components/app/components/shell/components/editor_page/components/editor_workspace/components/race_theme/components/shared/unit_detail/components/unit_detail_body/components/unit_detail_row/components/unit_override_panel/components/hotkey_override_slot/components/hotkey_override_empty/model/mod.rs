use super::view::HotkeyOverrideEmptyView;
use dioxus::prelude::*;

/// The prompt shown in the override panel before a tile is selected.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideEmptyModel {
    #[props(into)]
    pub message: String,
}

impl From<&HotkeyOverrideEmptyView> for HotkeyOverrideEmptyModel {
    fn from(view: &HotkeyOverrideEmptyView) -> Self {
        let HotkeyOverrideEmptyView { message } = view.clone();
        Self { message }
    }
}

impl ddd::Model for HotkeyOverrideEmptyModel {
    type View = HotkeyOverrideEmptyView;
}
