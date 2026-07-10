pub mod components;
mod logic;
mod style;

use components::layout_intro_line::LayoutIntroLine;
use dioxus::prelude::*;
use logic::intro_lines;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(LayoutIntro);
