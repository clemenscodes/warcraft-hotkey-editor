mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HelpTermProps;
use style::CLASS;
assert_component!(HelpTerm);

/// A glossary term heading. A leaf: the entry passes the term as children.
#[component]
pub fn HelpTerm(props: HelpTermProps) -> Element {
    let term = props.children.clone();
    rsx! {
        p { class: CLASS, {term} }
    }
}
