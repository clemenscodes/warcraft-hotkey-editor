mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HelpTermProps;

assert_component!(HelpTerm);

/// A glossary term heading. A leaf: the entry passes the term as children.
#[component]
pub fn HelpTerm(props: HelpTermProps) -> Element {
    let term = props.children.clone();
    rsx! {
        p {
            class: CLASS,
            {term}
        }
    }
}
