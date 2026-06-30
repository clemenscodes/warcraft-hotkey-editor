mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HelpBodyTextProps;

assert_component!(HelpBodyText);

/// A paragraph of explanatory help copy. A leaf reused by the resolver prose and
/// every glossary entry; the parent passes the text as children.
#[component]
pub fn HelpBodyText(props: HelpBodyTextProps) -> Element {
    rsx! {
        p {
            class: CLASS,
            {props.children}
        }
    }
}
