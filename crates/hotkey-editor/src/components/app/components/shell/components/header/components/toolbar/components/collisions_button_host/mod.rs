pub mod components;
mod presentation;
mod style;

use components::collisions_button::CollisionsButton;
use dioxus::prelude::*;
use presentation::use_collisions_button;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionsButtonHost() -> Element {
    let model = use_collisions_button();
    let summary = model.summary;
    let onclick = model.onclick;
    rsx! {
        div {
            class: CLASS,
            CollisionsButton {
                summary,
                onclick,
            }
        }
    }
}

assert_component!(CollisionsButtonHost);
