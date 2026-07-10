pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::shared::help_section_title::HelpSectionTitle;
use components::help_callout::{HelpCallout, HelpCalloutProps};
use dioxus::prelude::*;
pub use props::HelpWorkflowSectionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The left column of the top row: the heading above the numbered workflow in
/// its callout.
#[component]
pub fn HelpWorkflowSection(props: HelpWorkflowSectionProps) -> Element {
    let callout = HelpCalloutProps::from(&props);
    rsx! {
        section { class: CLASS,
            HelpSectionTitle { title: "The workflow" }
            HelpCallout { ..callout }
        }
    }
}

assert_component!(HelpWorkflowSection);
