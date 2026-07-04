pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::help_glossary_entry::HelpGlossaryEntry;
use dioxus::prelude::*;
pub use props::HelpGlossaryColumnProps;
use style::CLASS;
assert_component!(HelpGlossaryColumn);

/// One glossary: a stack of term-and-definition entries, one per term it is
/// handed.
#[component]
pub fn HelpGlossaryColumn(props: HelpGlossaryColumnProps) -> Element {
    let entries = props.entries;
    rsx! {
        div { class: CLASS,
            for entry in entries {
                HelpGlossaryEntry { key: "{entry.term}", ..entry.clone() }
            }
        }
    }
}
