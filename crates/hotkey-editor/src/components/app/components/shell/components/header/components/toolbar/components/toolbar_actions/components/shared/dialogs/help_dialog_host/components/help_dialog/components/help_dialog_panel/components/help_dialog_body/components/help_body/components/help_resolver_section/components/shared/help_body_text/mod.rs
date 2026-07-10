mod props;
mod view;

pub use view::HelpBodyTextView;
mod style;

use dioxus::prelude::*;
use props::HelpBodyTextProps;
use style::CLASS;
use tw_macro::assert_component;

/// A paragraph of explanatory help copy. A leaf reused by the resolver prose and
/// every glossary entry; the parent passes the text.
#[component]
pub fn HelpBodyText(props: HelpBodyTextProps) -> Element {
    let text = props.text.clone();
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(HelpBodyText);
