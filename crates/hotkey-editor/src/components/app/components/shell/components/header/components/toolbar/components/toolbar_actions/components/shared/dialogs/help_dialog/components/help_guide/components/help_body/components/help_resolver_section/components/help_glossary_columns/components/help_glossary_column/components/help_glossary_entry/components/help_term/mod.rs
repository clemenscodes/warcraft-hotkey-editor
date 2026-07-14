mod model;
mod view;

pub use view::HelpTermView;
mod style;

use dioxus::prelude::*;
use model::HelpTermModel;
use style::CLASS;
use tw_macro::assert_component;

/// A glossary term heading. A leaf: the entry passes the term text.
#[component]
pub fn HelpTerm(props: HelpTermModel) -> Element {
    let term = props.term.clone();
    rsx! {
        p {
            class: CLASS,
            {term}
        }
    }
}

assert_component!(HelpTerm);
