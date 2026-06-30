pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::help_legend_section::{HelpLegendSection, HelpLegendSectionProps};
use components::help_workflow_section::{HelpWorkflowSection, HelpWorkflowSectionProps};
use style::CLASS;

pub use props::HelpTopRowProps;

assert_component!(HelpTopRow);

/// The split upper region: the workflow on the left, the button legend on the
/// right; stacked on small screens, side by side on laptop and up.
#[component]
pub fn HelpTopRow(props: HelpTopRowProps) -> Element {
    let workflow = HelpWorkflowSectionProps::from(&props);
    let legend = HelpLegendSectionProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            HelpWorkflowSection { ..workflow }
            HelpLegendSection { ..legend }
        }
    }
}
