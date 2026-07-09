mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::RaceTabsProps;
use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::mode_and_race_tabs::components::race_tabs::components::shared::race_tab_state::{
    RaceTabState, RaceTabStateProps,
};
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::Race;
assert_component!(NightelfRaceTab);

/// The Night Elf race tab: publishes Night Elf's banner colour, accent, strength, and
/// banner image as the `--race-*`/`--banner-*` custom properties the shared chrome
/// reads, then forwards the navigation to the shared state dispatcher for its race. A
/// thin per-race modifier — its only difference from the other four tabs is the four
/// values in its `style.rs`.
#[component]
pub fn NightelfRaceTab(props: RaceTabsProps) -> Element {
    let navigation = props;
    let state = RaceTabStateProps {
        race: Race::Nightelf,
        navigation,
    };
    rsx! {
        div {
            class: CLASS,
            RaceTabState { ..state }
        }
    }
}
