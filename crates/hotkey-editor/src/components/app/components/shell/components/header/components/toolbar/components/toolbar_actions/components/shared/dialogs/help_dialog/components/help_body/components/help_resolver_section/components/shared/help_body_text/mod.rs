mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpBodyTextProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpBodyText);

/// A paragraph of explanatory help copy. A leaf reused by the resolver prose and
/// every glossary entry; the parent passes the text as children.
#[component]
pub fn HelpBodyText(props: HelpBodyTextProps) -> Element {
    rsx! {
        p { class: CLASS, {props.children} }
    }
}
