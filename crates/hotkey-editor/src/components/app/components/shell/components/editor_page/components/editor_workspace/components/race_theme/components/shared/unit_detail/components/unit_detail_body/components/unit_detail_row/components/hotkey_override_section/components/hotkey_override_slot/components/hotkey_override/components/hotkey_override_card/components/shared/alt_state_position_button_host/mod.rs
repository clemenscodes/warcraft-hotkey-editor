pub mod components;
mod model;
mod view;

pub use view::AltStatePositionButtonHostView;
mod style;

use components::alt_state_position_button::AltStatePositionButton;
use dioxus::prelude::*;
use model::AltStatePositionButtonHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AltStatePositionButtonHost(props: AltStatePositionButtonHostModel) -> Element {
    let title = props.title;
    let aria_label = props.aria_label;
    let on_click = props.on_click;
    rsx! {
        div {
            class: CLASS,
            AltStatePositionButton {
                title,
                aria_label,
                on_click,
            }
        }
    }
}

assert_component!(AltStatePositionButtonHost);
