mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_body_text::HelpBodyText;
use style::CLASS;

pub use props::HelpResolverProseProps;

assert_component!(HelpResolverProse);

/// The plain-language walkthrough of how the resolver settles a clash, one
/// paragraph per passage passed in.
#[component]
pub fn HelpResolverProse(props: HelpResolverProseProps) -> Element {
    rsx! {
        div {
            class: CLASS,
            for (index, paragraph) in props.paragraphs.iter().copied().enumerate() {
                HelpBodyText {
                    key: "{index}",
                    "{paragraph}"
                }
            }
        }
    }
}
