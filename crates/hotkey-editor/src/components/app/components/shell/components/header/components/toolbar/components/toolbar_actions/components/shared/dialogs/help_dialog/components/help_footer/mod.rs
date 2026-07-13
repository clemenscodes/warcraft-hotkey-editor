pub mod components;
mod model;
mod presentation;
mod style;
mod view;

pub use view::HelpFooterView;

use components::help_dismiss::HelpDismiss;
use dioxus::prelude::*;
use presentation::{HelpFooterPresentation, use_help_footer};
use style::CLASS;
use tw_macro::assert_component;

/// Connected, isolated footer content: the pinned bar below the scrolling guide, separated
/// from it by a gold rule, holding the dismiss button that closes the guide and records that
/// the player has seen it. A dialog places it as its footer region, so it stays put while
/// the guide scrolls above it.
#[component]
pub fn HelpFooter() -> Element {
    let HelpFooterPresentation { on_dismiss } = use_help_footer();
    rsx! {
        footer {
            class: CLASS,
            HelpDismiss { on_dismiss }
        }
    }
}

assert_component!(HelpFooter);
