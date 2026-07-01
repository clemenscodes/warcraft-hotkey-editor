pub mod components;
mod style;

use super::super::data::INTRO_LINES;
use crate::assert_component;
use components::layout_intro_line::LayoutIntroLine;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(LayoutIntro);

/// The instruction block above the grid: one line per entry in the intro data.
#[component]
pub fn LayoutIntro() -> Element {
    rsx! {
        div { class: CLASS,
            for &line in INTRO_LINES {
                LayoutIntroLine { line: line
                            .to_string() }
            }
        }
    }
}
