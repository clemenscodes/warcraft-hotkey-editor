pub mod components;
mod model;
mod view;

pub use view::HelpWorkflowView;
mod style;

use components::help_workflow_step::HelpWorkflowStep;
use dioxus::prelude::*;
use model::HelpWorkflowModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpWorkflow(props: HelpWorkflowModel) -> Element {
    rsx! {
        ol {
            class: CLASS,
            for (index, segments) in props.steps.iter().copied().enumerate() {
                HelpWorkflowStep {
                    key: "{index}",
                    segments,
                }
            }
        }
    }
}

assert_component!(HelpWorkflow);
