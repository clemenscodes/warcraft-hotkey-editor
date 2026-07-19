pub mod components;
mod model;

use components::active_race_tab::ActiveRaceTab;
use components::inactive_race_tab::InactiveRaceTab;
use dioxus::prelude::*;
use model::RaceTabStateModel;
use tw_macro::assert_component;

#[component]
pub fn RaceTabState(props: RaceTabStateModel) -> Element {
    let is_active = props.is_active;
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match is_active {
        true => rsx! {
            ActiveRaceTab {
                label,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            InactiveRaceTab {
                label,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(RaceTabState);
