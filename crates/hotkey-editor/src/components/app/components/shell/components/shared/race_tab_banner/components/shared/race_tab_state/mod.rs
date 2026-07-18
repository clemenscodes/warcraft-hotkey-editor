pub mod components;
mod presentation;

use crate::components::app::components::shell::components::shared::race_tab_banner::binding::RaceTabBinding;
use components::active_race_tab::ActiveRaceTab;
use components::inactive_race_tab::InactiveRaceTab;
use dioxus::prelude::*;
use presentation::RaceTabBehavior;
use tw_macro::assert_component;

#[component]
pub fn RaceTabState(props: RaceTabBinding) -> Element {
    let behavior = RaceTabBehavior::from(&props);
    let label = behavior.label().to_string();
    let onclick = behavior.onclick();
    let onkeydown = behavior.onkeydown();
    match behavior.is_active() {
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
