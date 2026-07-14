mod model;
mod view;

pub use view::HelpResolverProseView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::components::help_body::components::help_resolver_section::components::shared::help_body_text::HelpBodyText;
use dioxus::prelude::*;
use model::HelpResolverProseModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpResolverProse(props: HelpResolverProseModel) -> Element {
    rsx! {
        div {
            class: CLASS,
            for (index, paragraph) in props.paragraphs.iter().copied().enumerate() {
                HelpBodyText {
                    key: "{index}",
                    text: paragraph,
                }
            }
        }
    }
}

assert_component!(HelpResolverProse);
