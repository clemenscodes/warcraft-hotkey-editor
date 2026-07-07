pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_hotkeys_section::{
    SystemHotkeysSection, SystemHotkeysSectionProps,
};
use dioxus::prelude::*;
pub use props::HeroSelectionHotkeysViewProps;

/// The hero-selection hotkey editor: three big slots for selecting heroes by index.
use tw_macro::assert_component;
assert_component!(HeroSelectionHotkeysView);
#[component]
pub fn HeroSelectionHotkeysView(props: HeroSelectionHotkeysViewProps) -> Element {
    rsx! {
        SystemHotkeysSection { ..SystemHotkeysSectionProps::from(&props) }
    }
}
