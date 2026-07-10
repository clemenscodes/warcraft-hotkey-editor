pub mod components;
mod props;
mod style;

use components::help_glossary_column::HelpGlossaryColumn;
use dioxus::prelude::*;
use props::HelpGlossaryColumnsProps;
use style::CLASS;
use tw_macro::assert_component;

/// The side-by-side glossaries that define the resolver's vocabulary, one column
/// per entry list passed in.
#[component]
pub fn HelpGlossaryColumns(props: HelpGlossaryColumnsProps) -> Element {
    rsx! {
        div { class: CLASS,
            for (index, entries) in props.columns.iter().copied().enumerate() {
                HelpGlossaryColumn { key: "{index}", entries }
            }
        }
    }
}

assert_component!(HelpGlossaryColumns);
