mod props;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::shared::race_tab_state::components::shared::race_tab::{
    RaceTab, RaceTabProps,
};
use dioxus::prelude::*;
use tw_macro::assert_component;
pub use props::InactiveRaceTabProps;

/// The inactive race tab: a named alias that renders the base `RaceTab` as-is and adds
/// nothing. It exists only so the dispatcher reads as a clean `if active { ActiveRaceTab }
/// else { InactiveRaceTab }` and so each state has its own component. It owns no class —
/// the base's resting look is already the inactive look.
#[component]
pub fn InactiveRaceTab(props: InactiveRaceTabProps) -> Element {
    let base = RaceTabProps::from(&props);
    rsx! {
        RaceTab { ..base }
    }
}

assert_component!(InactiveRaceTab);
