pub mod components;
mod presentation;
mod style;

use components::layout_intro_line::LayoutIntroLine;
use dioxus::prelude::*;
use presentation::intro_lines;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn LayoutIntro() -> Element {
    let lines = intro_lines();
    rsx! {
        div {
            class: CLASS,
            for entry in lines {
                LayoutIntroLine {
                    line: entry.line,
                }
            }
        }
    }
}

assert_component!(LayoutIntro);
