use super::view::HotkeysContentView;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

/// The shared-hotkey two-pane content: the clashing units the sidebar and the hotkey
/// unit detail pane both render.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeysContentModel {
    pub units: Vec<HotkeyUnitView>,
}

impl From<&HotkeysContentView> for HotkeysContentModel {
    fn from(view: &HotkeysContentView) -> Self {
        let HotkeysContentView { units } = view.clone();
        Self { units }
    }
}

impl ddd::Model for HotkeysContentModel {
    type View = HotkeysContentView;
}
