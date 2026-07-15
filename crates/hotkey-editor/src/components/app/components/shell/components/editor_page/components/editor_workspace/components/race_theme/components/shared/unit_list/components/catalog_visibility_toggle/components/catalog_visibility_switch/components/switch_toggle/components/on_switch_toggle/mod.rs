mod model;
mod view;

pub use view::OnSwitchToggleView;
mod style;

use super::shared::switch_knob::SwitchKnob;
use dioxus::prelude::*;
use model::OnSwitchToggleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn OnSwitchToggle(props: OnSwitchToggleModel) -> Element {
    let title = props.title;
    let aria_label = props.aria_label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "switch",
            aria_checked: true,
            aria_label,
            title,
            onclick,
            SwitchKnob {}
        }
    }
}

assert_component!(OnSwitchToggle);
