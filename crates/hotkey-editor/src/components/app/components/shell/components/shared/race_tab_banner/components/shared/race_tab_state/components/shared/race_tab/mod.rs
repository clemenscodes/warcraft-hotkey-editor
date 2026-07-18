pub mod components;
mod model;
mod view;

pub use view::RaceTabView;
mod style;

use components::race_tab_label::RaceTabLabel;
use dioxus::prelude::*;
use model::RaceTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceTab(props: RaceTabModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            onclick,
            onkeydown,
            RaceTabLabel {
                label,
            }
        }
    }
}

assert_component!(RaceTab);
