mod model;
mod presentation;
mod view;

pub use view::ToggleButtonView;
mod style;

use dioxus::prelude::*;
use model::ToggleButtonModel;
use presentation::ToggleButtonPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToggleButton(props: ToggleButtonModel) -> Element {
    let ToggleButtonPresentation {
        label,
        title,
        pressed,
        onclick,
        onkeydown,
    } = ToggleButtonPresentation::from(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: pressed,
            title,
            onclick,
            onkeydown,
            {label}
        }
    }
}

assert_component!(ToggleButton);
