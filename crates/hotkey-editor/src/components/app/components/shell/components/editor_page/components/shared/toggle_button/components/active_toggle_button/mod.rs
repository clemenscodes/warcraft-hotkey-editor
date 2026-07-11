mod model;
mod view;

pub use view::ActiveToggleButtonView;
mod style;

use dioxus::prelude::*;
use model::ActiveToggleButtonModel;
use style::CLASS;
use tw_macro::assert_component;

/// The active toggle button: the shared gold pill lit to show it is the current choice
/// in its group. Presentational — the dispatcher renders it for the one active button.
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
