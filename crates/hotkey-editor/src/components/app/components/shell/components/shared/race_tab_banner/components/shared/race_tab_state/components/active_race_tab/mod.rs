pub mod components;
mod model;
mod view;

pub use view::ActiveRaceTabView;
mod style;

use crate::components::app::components::shell::components::shared::race_tab_banner::components::shared::race_tab_state::components::shared::race_tab::RaceTab;
use components::active_accent::ActiveAccent;
use dioxus::prelude::*;
use model::ActiveRaceTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveRaceTab(props: ActiveRaceTabModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        div {
            class: CLASS,
            RaceTab {
                label,
                onclick,
                onkeydown,
            }
            ActiveAccent {}
        }
    }
}

assert_component!(ActiveRaceTab);
