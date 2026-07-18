mod style;

use crate::components::app::components::shell::components::shared::race_tab_banner::binding::RaceTabBinding;
use crate::components::app::components::shell::components::shared::race_tab_banner::components::shared::race_tab_state::RaceTabState;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NightelfRaceTab(props: RaceTabBinding) -> Element {
    let is_active = props.is_active;
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        div {
            class: CLASS,
            RaceTabState {
                is_active,
                label,
                onclick,
                onkeydown,
            }
        }
    }
}

assert_component!(NightelfRaceTab);
