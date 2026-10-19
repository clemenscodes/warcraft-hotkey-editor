pub mod components;
mod style;

use components::max_heading::MaxHeading;
use components::regeneration_heading::RegenerationHeading;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

const MAX_TEXT: &str = "Max";
const REGENERATION_TEXT: &str = "Regeneration";

#[component]
pub fn VitalityHeadingRow() -> Element {
    rsx! {
        div {
            class: CLASS,
            MaxHeading {
                text: MAX_TEXT,
            }
            RegenerationHeading {
                text: REGENERATION_TEXT,
            }
        }
    }
}

assert_component!(VitalityHeadingRow);
