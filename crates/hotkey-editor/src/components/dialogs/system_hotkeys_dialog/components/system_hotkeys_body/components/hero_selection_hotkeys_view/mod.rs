pub mod components;
mod logic;
mod props;

use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_hotkeys_section::{
    SystemHotkeysSection, SystemHotkeysSectionProps,
};
use dioxus::prelude::*;
pub use props::HeroSelectionHotkeysViewProps;

/// The hero-selection hotkey editor: three big slots for selecting heroes by index.
#[component]
pub fn HeroSelectionHotkeysView(props: HeroSelectionHotkeysViewProps) -> Element {
    rsx! {
        SystemHotkeysSection { ..SystemHotkeysSectionProps::from(&props) }
    }
}
