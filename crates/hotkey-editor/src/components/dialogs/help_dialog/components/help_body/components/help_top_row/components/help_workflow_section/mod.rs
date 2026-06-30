pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_section_title::HelpSectionTitle;
use components::help_callout::{HelpCallout, HelpCalloutProps};
use style::CLASS;

pub use props::HelpWorkflowSectionProps;

assert_component!(HelpWorkflowSection);

/// The left column of the top row: the heading above the numbered workflow in
/// its callout.
#[component]
pub fn HelpWorkflowSection(props: HelpWorkflowSectionProps) -> Element {
    let callout = HelpCalloutProps::from(&props);
    rsx! {
        section {
            class: CLASS,
            HelpSectionTitle { "The workflow" }
            HelpCallout { ..callout }
        }
    }
}
