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

#[component]
pub fn HelpGuide() -> Element {
    let HelpGuidePresentation { content } = use_help_guide();
    rsx! {
        div {
            class: CLASS,
            HelpBody {
                content,
            }
        }
    }
}

assert_component!(HelpGuide);
