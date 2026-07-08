pub mod components;
mod props;
mod style;

use components::help_workflow_step::HelpWorkflowStep;
use dioxus::prelude::*;
pub use props::HelpWorkflowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpWorkflow);

/// The ordered list of workflow steps. A pure loop: the steps and their inline
/// glyphs are data, passed in, never baked into this markup.
#[component]
pub fn HelpWorkflow(props: HelpWorkflowProps) -> Element {
    rsx! {
        ol { class: CLASS,
            for (index, segments) in props.steps.iter().copied().enumerate() {
                HelpWorkflowStep { key: "{index}", segments }
            }
        }
    }
}
