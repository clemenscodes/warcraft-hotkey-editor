pub mod components;
mod model;
mod style;
mod view;

pub use view::FilterSwitchView;

use components::filter_switch_knob::FilterSwitchKnob;
use dioxus::prelude::*;
use model::FilterSwitchModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilterSwitch(props: FilterSwitchModel) -> Element {
    let FilterSwitchModel { is_on, onclick } = props;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: is_on,
            aria_label: "Toggle",
            onclick,
            FilterSwitchKnob {}
        }
    }
}

assert_component!(FilterSwitch);
