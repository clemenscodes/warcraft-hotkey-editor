mod model;
mod view;

pub use view::AbilityTierButtonView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AbilityTierButtonModel;

#[component]
pub fn AbilityTierButton(props: AbilityTierButtonModel) -> Element {
    let aria_label = props.aria_label;
    let icon = props.icon;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            aria_label,
            onclick: on_click,
            span {
                aria_hidden: "true",
                dangerous_inner_html: icon,
            }
        }
    }
}

assert_component!(AbilityTierButton);
