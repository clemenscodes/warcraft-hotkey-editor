pub mod components;
mod props;
mod style;

use components::help_glossary_entry::HelpGlossaryEntry;
use dioxus::prelude::*;
use props::HelpGlossaryColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// One glossary: a stack of term-and-definition entries, one per item it is
/// handed.
#[component]
pub fn HelpGlossaryColumn(props: HelpGlossaryColumnProps) -> Element {
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
