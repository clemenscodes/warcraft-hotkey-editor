pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::HelpFooterHostView;

use components::help_dismiss::HelpDismiss;
use dioxus::prelude::*;
use presentation::{HelpFooterHostPresentation, use_help_footer_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connected, isolated footer content: the pinned bar below the scrolling guide, separated
/// from it by a gold rule, holding the dismiss button that closes the guide and records that
/// the player has seen it. A dialog places it as its footer region, so it stays put while
/// the guide scrolls above it.
#[component]
pub fn HelpFooterHost() -> Element {
    let HelpFooterHostPresentation { on_dismiss } = use_help_footer_host();
    rsx! {
        footer {
            class: CLASS,
            HelpDismiss { on_dismiss }
        }
    }
}

assert_component!(HelpFooterHost);
