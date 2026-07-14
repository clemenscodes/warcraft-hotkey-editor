mod model;
mod view;

pub use view::ActiveToggleButtonView;
mod style;

use dioxus::prelude::*;
use model::ActiveToggleButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveToggleButton(props: ActiveToggleButtonModel) -> Element {
    let label = props.label;
    let title = props.title;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: true,
            title,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(ActiveToggleButton);
