mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HelpSectionTitleProps;

assert_component!(HelpSectionTitle);

/// A section heading inside the help guide. A leaf: each section passes its
/// heading text as children.
#[component]
pub fn HelpSectionTitle(props: HelpSectionTitleProps) -> Element {
    rsx! {
        h3 {
            class: CLASS,
            {props.children}
        }
    }
}
