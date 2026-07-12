pub mod components;
pub mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::HelpGuideHostView;

use components::help_body::HelpBody;
use dioxus::prelude::*;
use presentation::{HelpGuideHostPresentation, use_help_guide_host};
use style::CLASS;
use tw_macro::assert_component;

/// Connected, isolated onboarding-guide content: the scrolling guide body that sources the
/// static guide content and feeds it to `HelpBody`. Zero dialog chrome — it is
/// page-renderable on its own, and a dialog places it as its body region. Its root is the
/// scrolling box that fills whatever space it is given, above the pinned footer.
#[component]
pub fn HelpGuideHost() -> Element {
    let HelpGuideHostPresentation { content } = use_help_guide_host();
    rsx! {
        div {
            class: CLASS,
            HelpBody { content }
        }
    }
}

assert_component!(HelpGuideHost);
