pub mod components;
mod style;

use components::mode_tabs_host::ModeTabsHost;
use components::race_tabs_host::RaceTabsHost;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EditorTabsBar);

/// The editor's top navigation bar: the game-mode column beside the race tabs. Pure
/// layout — each side is a connected host that sources its one concern (the active mode,
/// the active race) from context, so this bar threads no props at all.
#[component]
pub fn EditorTabsBar() -> Element {
    rsx! {
        div {
            class: CLASS,
            ModeTabsHost {}
            RaceTabsHost {}
        }
    }
}
