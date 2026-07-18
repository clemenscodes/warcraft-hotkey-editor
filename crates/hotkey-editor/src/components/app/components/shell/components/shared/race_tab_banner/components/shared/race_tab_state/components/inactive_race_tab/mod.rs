mod model;
mod view;

pub use view::InactiveRaceTabView;

use crate::components::app::components::shell::components::shared::race_tab_banner::components::shared::race_tab_state::components::shared::race_tab::RaceTab;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::InactiveRaceTabModel;

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
