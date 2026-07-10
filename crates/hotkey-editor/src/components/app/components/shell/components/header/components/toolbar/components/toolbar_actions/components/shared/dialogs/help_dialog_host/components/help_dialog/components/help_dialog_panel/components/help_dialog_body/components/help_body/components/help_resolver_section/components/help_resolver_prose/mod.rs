mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::help_resolver_section::components::shared::help_body_text::HelpBodyText;
use dioxus::prelude::*;
pub use props::HelpResolverProseProps;
use style::CLASS;
use tw_macro::assert_component;

/// The plain-language walkthrough of how the resolver settles a clash, one
/// paragraph per passage passed in.
#[component]
pub fn HelpResolverProse(props: HelpResolverProseProps) -> Element {
    rsx! {
        div { class: CLASS,
            for (index, paragraph) in props.paragraphs.iter().copied().enumerate() {
                HelpBodyText { key: "{index}", text: paragraph }
            }
        }
    }
}

assert_component!(HelpResolverProse);
