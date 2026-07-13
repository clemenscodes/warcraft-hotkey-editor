pub mod components;
pub mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::HelpGuideView;

use components::help_body::HelpBody;
use dioxus::prelude::*;
use presentation::{HelpGuidePresentation, use_help_guide};
use style::CLASS;
use tw_macro::assert_component;

/// Connected, isolated onboarding-guide content: the scrolling guide body that sources the
/// static guide content and feeds it to `HelpBody`. Zero dialog chrome — it is
/// page-renderable on its own, and a dialog places it as its body region. Its root is the
/// scrolling box that fills whatever space it is given, above the pinned footer.
#[component]
pub fn HelpGuide() -> Element {
    let HelpGuidePresentation { content } = use_help_guide();
    rsx! {
        div {
            class: CLASS,
            HelpBody { content }
        }
    }
}

assert_component!(HelpGuide);
