pub mod components;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_body_text::HelpBodyText;
use components::help_term::HelpTerm;
use dioxus::prelude::*;
pub use props::HelpGlossaryEntryProps;
use style::CLASS;
assert_component!(HelpGlossaryEntry);

/// One glossary term paired with its definition.
#[component]
pub fn HelpGlossaryEntry(props: HelpGlossaryEntryProps) -> Element {
    let term = props.term;
    let description = props.description;
    rsx! {
        div { class: CLASS,
            HelpTerm { "{term}" }
            HelpBodyText { "{description}" }
        }
    }
}
