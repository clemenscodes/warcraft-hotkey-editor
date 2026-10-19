pub mod components;
mod model;
mod view;

pub use view::HelpGlossaryEntryView;
mod style;

use components::help_body_text::HelpBodyText;
use components::help_term::HelpTerm;
use dioxus::prelude::*;
use model::HelpGlossaryEntryModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpGlossaryEntry(props: HelpGlossaryEntryModel) -> Element {
    let item = props.item;
    let term = item.term();
    let description = item.description();
    rsx! {
        div {
            class: CLASS,
            HelpTerm {
                term,
            }
            HelpBodyText {
                text: description,
            }
        }
    }
}

assert_component!(HelpGlossaryEntry);
