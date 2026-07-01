pub mod components;
mod props;

use crate::assert_component;
use crate::components::dialogs::system_hotkeys_dialog::components::system_hotkeys_section::SystemHotkeysSection;
use components::hero_selection_row::HeroSelectionRow;
use dioxus::prelude::*;
pub use props::HeroSelectionHotkeysViewProps;
assert_component!(HeroSelectionHotkeysView);

/// The hero-selection hotkey editor: three big slots for selecting heroes by index.
#[component]
pub fn HeroSelectionHotkeysView(props: HeroSelectionHotkeysViewProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let editing_section = props.editing_section;
    rsx! {
        SystemHotkeysSection { intro: "Hotkeys for selecting your heroes by index.",
            HeroSelectionRow { loaded_keys, editing_section }
        }
    }
}
