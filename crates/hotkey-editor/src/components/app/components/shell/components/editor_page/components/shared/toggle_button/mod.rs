pub mod components;
mod model;
mod view;

pub use view::ToggleButtonView;

use components::active_toggle_button::ActiveToggleButton;
use components::idle_toggle_button::IdleToggleButton;
use dioxus::prelude::*;
use model::ToggleButtonModel;
use tw_macro::assert_component;

#[component]
pub fn ToggleButton(props: ToggleButtonModel) -> Element {
    let label = props.label;
    let title = props.title;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match props.active {
        true => rsx! {
            ActiveToggleButton {
                label,
                title,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            IdleToggleButton {
                label,
                title,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(ToggleButton);
