mod model;
mod view;

pub use view::IdleToggleButtonView;
mod style;

use dioxus::prelude::*;
use model::IdleToggleButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleToggleButton(props: IdleToggleButtonModel) -> Element {
    let label = props.label;
    let title = props.title;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: false,
            title,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(IdleToggleButton);
