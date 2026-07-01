pub mod components;
mod props;
mod style;

use crate::assert_component;
use crate::components::dialogs::help_dialog::components::help_section_title::HelpSectionTitle;
use components::help_callout::{HelpCallout, HelpCalloutProps};
use dioxus::prelude::*;
pub use props::HelpWorkflowSectionProps;
use style::CLASS;
assert_component!(HelpWorkflowSection);

/// The left column of the top row: the heading above the numbered workflow in
/// its callout.
#[component]
pub fn HelpWorkflowSection(props: HelpWorkflowSectionProps) -> Element {
    let callout = HelpCalloutProps::from(&props);
    rsx! {
        section { class: CLASS,
            HelpSectionTitle { "The workflow" }
            HelpCallout { ..callout }
        }
    }
}
