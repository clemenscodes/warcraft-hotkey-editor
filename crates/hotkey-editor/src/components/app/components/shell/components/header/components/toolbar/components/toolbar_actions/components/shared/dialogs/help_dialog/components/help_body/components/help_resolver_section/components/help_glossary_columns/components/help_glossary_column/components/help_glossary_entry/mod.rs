pub mod components;
mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_body::components::help_resolver_section::components::shared::help_body_text::HelpBodyText;
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
