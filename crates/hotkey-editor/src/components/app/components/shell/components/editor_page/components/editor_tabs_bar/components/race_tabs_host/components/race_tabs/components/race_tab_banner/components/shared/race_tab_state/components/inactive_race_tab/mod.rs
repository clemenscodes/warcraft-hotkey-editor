mod model;
mod view;

pub use view::InactiveRaceTabView;

use crate::components::app::components::shell::components::editor_page::components::editor_tabs_bar::components::race_tabs_host::components::race_tabs::components::race_tab_banner::components::shared::race_tab_state::components::shared::race_tab::RaceTab;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::InactiveRaceTabModel;

/// The inactive race tab: a named alias that renders the base `RaceTab` as-is and adds
/// nothing. It exists only so the dispatcher reads as a clean `if active { ActiveRaceTab }
/// else { InactiveRaceTab }` and so each state has its own component. It owns no class —
/// the base's resting look is already the inactive look.
#[component]
pub fn InactiveRaceTab(props: InactiveRaceTabModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        RaceTab {
            label,
            onclick,
            onkeydown,
        }
    }
}

assert_component!(InactiveRaceTab);
