mod props;
mod style;

use crate::assert_component;
use crate::components::tabs::mode_and_race_tabs::{ModeAndRaceTabs, ModeAndRaceTabsProps};
use dioxus::prelude::*;
pub use props::EditorTabsBarProps;
use style::CLASS;
assert_component!(EditorTabsBar);

/// The editor's top bar: the game-mode toggle beside the race tabs.
#[component]
pub fn EditorTabsBar(props: EditorTabsBarProps) -> Element {
    let tabs = ModeAndRaceTabsProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            ModeAndRaceTabs { ..tabs }
        }
    }
}
