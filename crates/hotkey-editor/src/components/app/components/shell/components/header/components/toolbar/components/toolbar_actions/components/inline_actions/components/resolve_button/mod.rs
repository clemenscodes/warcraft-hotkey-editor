mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::inline_actions::components::shared::toolbar_button::ToolbarButton;
use dioxus::prelude::*;
use presentation::{ResolveButtonPresentation, use_resolve_button};
use style::CLASS;
use tw_macro::assert_component;

/// The inline resolve action: the toolbar button that navigates to the conflict-resolution view.
/// It sources its own disabled state and click handler from the navigation service and document.
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
