pub mod components;
mod logic;
mod style;

use crate::assert_component;
use components::layout_intro_line::LayoutIntroLine;
use dioxus::prelude::*;
use logic::intro_lines;
use style::CLASS;
assert_component!(LayoutIntro);

/// The instruction block above the grid: one line per entry in the intro data.
#[component]
pub fn LayoutIntro() -> Element {
    let lines = intro_lines();
    rsx! {
        div {
            class: CLASS,
            for entry in lines {
                LayoutIntroLine { ..entry }
            }
        }
    }
}
