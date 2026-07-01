mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HelpBodyTextProps;
use style::CLASS;
assert_component!(HelpBodyText);

/// A paragraph of explanatory help copy. A leaf reused by the resolver prose and
/// every glossary entry; the parent passes the text as children.
#[component]
pub fn HelpBodyText(props: HelpBodyTextProps) -> Element {
    rsx! {
        p { class: CLASS, {props.children} }
    }
}
