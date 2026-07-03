mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_body::components::help_resolver_section::components::shared::help_body_text::HelpBodyText;
use dioxus::prelude::*;
pub use props::HelpResolverProseProps;
use style::CLASS;
assert_component!(HelpResolverProse);

/// The plain-language walkthrough of how the resolver settles a clash, one
/// paragraph per passage passed in.
#[component]
pub fn HelpResolverProse(props: HelpResolverProseProps) -> Element {
    rsx! {
        div { class: CLASS,
            for (index, paragraph) in props.paragraphs.iter().copied().enumerate() {
                HelpBodyText { key: "{index}", "{paragraph}" }
            }
        }
    }
}
