pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::help_glossary_column::HelpGlossaryColumn;
use style::CLASS;

pub use props::HelpGlossaryColumnsProps;

assert_component!(HelpGlossaryColumns);

/// The side-by-side glossaries that define the resolver's vocabulary, one column
/// per entry list passed in.
#[component]
pub fn HelpGlossaryColumns(props: HelpGlossaryColumnsProps) -> Element {
    rsx! {
        div {
            class: CLASS,
            for (index, entries) in props.columns.iter().copied().enumerate() {
                HelpGlossaryColumn {
                    key: "{index}",
                    entries,
                }
            }
        }
    }
}
