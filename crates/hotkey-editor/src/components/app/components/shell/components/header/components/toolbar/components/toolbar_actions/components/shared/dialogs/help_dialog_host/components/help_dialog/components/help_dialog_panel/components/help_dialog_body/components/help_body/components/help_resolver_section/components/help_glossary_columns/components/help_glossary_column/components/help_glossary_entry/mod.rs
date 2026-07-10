pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::help_resolver_section::components::shared::help_body_text::HelpBodyText;
use components::help_term::HelpTerm;
use dioxus::prelude::*;
use props::HelpGlossaryEntryProps;
use style::CLASS;
use tw_macro::assert_component;

/// One glossary term paired with its definition.
#[component]
pub fn HelpGlossaryEntry(props: HelpGlossaryEntryProps) -> Element {
    let item = props.item;
    let term = item.term();
    let description = item.description();
    rsx! {
        div { class: CLASS,
            HelpTerm { term }
            HelpBodyText { text: description }
        }
    }
}

assert_component!(HelpGlossaryEntry);
