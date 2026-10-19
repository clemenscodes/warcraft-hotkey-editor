mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use presentation::{ResolveButtonPresentation, use_resolve_button};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolveButton() -> Element {
    let ResolveButtonPresentation {
        icon,
        aria_label,
        disabled,
        onclick,
    } = use_resolve_button();
    rsx! {
        div {
            class: CLASS,
            ToolbarButton {
                icon,
                aria_label,
                disabled,
                onclick,
            }
        }
    }
}

assert_component!(ResolveButton);
