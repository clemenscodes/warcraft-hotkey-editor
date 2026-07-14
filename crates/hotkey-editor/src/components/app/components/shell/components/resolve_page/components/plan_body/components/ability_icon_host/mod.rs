pub mod components;
mod model;
mod view;

pub use view::AbilityIconHostView;
mod style;

use components::ability_icon::AbilityIcon;
use dioxus::prelude::*;
use model::AbilityIconHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AbilityIconHost(props: AbilityIconHostModel) -> Element {
    let name = props.name;
    let icon_url = props.icon_url;
    let carrier_count = props.carrier_count;
    let is_winner = props.is_winner;
    let disabled = props.disabled;
    let inspected = props.inspected;
    rsx! {
        div {
            class: CLASS,
            AbilityIcon {
                name,
                icon_url,
                carrier_count,
                is_winner,
                disabled,
                inspected,
            }
        }
    }
}

assert_component!(AbilityIconHost);
