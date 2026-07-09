pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::{
    RaceTab, RaceTabProps,
};
use components::active_accent::ActiveAccent;
use dioxus::prelude::*;
pub use props::ActiveRaceTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ActiveRaceTab);

/// The active race tab: the base look, extended by composition. It renders the base
/// `RaceTab` unchanged and adds the accent ON TOP — the `ActiveAccent` overlay ring, plus
/// the `--label-color` its own root publishes so the base's label reads accent. It never
/// names or reuses the base's classes; it only stacks its own on top.
#[component]
pub fn ActiveRaceTab(props: ActiveRaceTabProps) -> Element {
    let base = RaceTabProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            RaceTab { ..base }
            ActiveAccent {}
        }
    }
}
