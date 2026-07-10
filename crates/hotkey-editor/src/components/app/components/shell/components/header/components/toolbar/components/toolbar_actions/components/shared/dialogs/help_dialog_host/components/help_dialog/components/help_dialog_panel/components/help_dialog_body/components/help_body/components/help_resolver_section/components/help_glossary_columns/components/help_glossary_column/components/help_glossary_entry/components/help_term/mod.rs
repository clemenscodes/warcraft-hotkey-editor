mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpTermProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpTerm);

/// A glossary term heading. A leaf: the entry passes the term text.
#[component]
pub fn HelpTerm(props: HelpTermProps) -> Element {
    let term = props.term.clone();
    rsx! {
        p { class: CLASS, {term} }
    }
}
