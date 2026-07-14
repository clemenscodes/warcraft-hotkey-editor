pub mod components;
mod model;
mod view;

pub use view::HelpTopRowView;
mod style;

use components::help_legend_section::HelpLegendSection;
use components::help_workflow_section::HelpWorkflowSection;
use dioxus::prelude::*;
use model::HelpTopRowModel;
use style::CLASS;
use tw_macro::assert_component;

/// The split upper region: the workflow on the left, the button legend on the
/// right; stacked on small screens, side by side on laptop and up.
#[component]
pub fn HelpTopRow(props: HelpTopRowModel) -> Element {
    let content = props.content;
    let steps = content.workflow();
    let rows = content.legend();
    rsx! {
        div {
            class: CLASS,
            HelpWorkflowSection {
                steps,
            }
            HelpLegendSection {
                rows,
            }
        }
    }
}

assert_component!(HelpTopRow);
