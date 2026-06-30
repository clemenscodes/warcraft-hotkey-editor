pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::help_workflow_step::HelpWorkflowStep;
use style::CLASS;

pub use props::HelpWorkflowProps;

assert_component!(HelpWorkflow);

/// The ordered list of workflow steps. A pure loop: the steps and their inline
/// glyphs are data, passed in, never baked into this markup.
#[component]
pub fn HelpWorkflow(props: HelpWorkflowProps) -> Element {
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
