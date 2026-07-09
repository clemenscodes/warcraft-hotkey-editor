mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::RaceTabBinding;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::RaceTabState;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NeutralRaceTab);

/// The Neutral race tab: publishes Neutral's banner colour, accent, strength, and
/// banner image as the `--race-*`/`--banner-*` custom properties the shared chrome reads,
/// then renders the shared state dispatcher with its finished binding. A thin per-race
/// modifier — its only difference from the other four tabs is the four values in its
/// `style.rs`; the binding it forwards carries no navigation.
#[component]
pub fn NeutralRaceTab(props: RaceTabBinding) -> Element {
    rsx! {
        div {
            class: CLASS,
            RaceTabState { ..props }
        }
    }
}
