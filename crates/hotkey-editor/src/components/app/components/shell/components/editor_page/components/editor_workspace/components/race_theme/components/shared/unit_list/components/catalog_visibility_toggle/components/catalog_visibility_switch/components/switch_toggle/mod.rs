pub mod components;
mod model;
mod view;

pub use view::SwitchToggleView;

use components::off_switch_toggle::OffSwitchToggle;
use components::on_switch_toggle::OnSwitchToggle;
use dioxus::prelude::*;
use model::SwitchToggleModel;
use tw_macro::assert_component;

#[component]
pub fn SwitchToggle(props: SwitchToggleModel) -> Element {
    let title = props.title;
    let aria_label = props.aria_label;
    let onclick = props.onclick;
    match props.is_on {
        true => rsx! {
            OnSwitchToggle {
                title,
                aria_label,
                onclick,
            }
        },
        false => rsx! {
            OffSwitchToggle {
                title,
                aria_label,
                onclick,
            }
        },
    }
}

assert_component!(SwitchToggle);
