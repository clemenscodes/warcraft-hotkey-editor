pub mod components;
mod model;
mod view;

pub use view::HelpGlossaryColumnsView;
mod style;

use components::help_glossary_column::HelpGlossaryColumn;
use dioxus::prelude::*;
use model::HelpGlossaryColumnsModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpGlossaryColumns(props: HelpGlossaryColumnsModel) -> Element {
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

assert_component!(HelpGlossaryColumns);
