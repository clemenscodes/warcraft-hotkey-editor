pub mod components;
mod model;
mod view;

pub use view::HelpGlossaryColumnView;
mod style;

use components::help_glossary_entry::HelpGlossaryEntry;
use dioxus::prelude::*;
use model::HelpGlossaryColumnModel;
use style::CLASS;
use tw_macro::assert_component;

/// One glossary: a stack of term-and-definition entries, one per item it is
/// handed.
#[component]
pub fn HelpGlossaryColumn(props: HelpGlossaryColumnModel) -> Element {
    let entries = props.entries;
    rsx! {
        div { class: CLASS,
            for (index, item) in entries.iter().copied().enumerate() {
                HelpGlossaryEntry {
                    key: "{index}",
                    item,
                }
            }
        }
    }
}

assert_component!(HelpGlossaryColumn);
