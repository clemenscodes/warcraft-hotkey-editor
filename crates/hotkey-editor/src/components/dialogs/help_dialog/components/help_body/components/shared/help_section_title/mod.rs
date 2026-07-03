mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HelpSectionTitleProps;
use style::CLASS;
assert_component!(HelpSectionTitle);

/// A section heading inside the help guide. A leaf: each section passes its
/// heading text as children.
#[component]
pub fn HelpSectionTitle(props: HelpSectionTitleProps) -> Element {
    rsx! {
        h3 { class: CLASS, {props.children} }
    }
}
