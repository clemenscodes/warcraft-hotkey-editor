pub mod components;
mod model;
mod view;

pub use view::HelpWorkflowStepView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::components::help_body::components::help_top_row::components::help_workflow_section::components::help_callout::components::help_workflow::components::help_workflow_step::components::help_inline_icon::HelpInlineIcon;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpSegment;
use dioxus::prelude::*;
use model::HelpWorkflowStepModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpWorkflowStep(props: HelpWorkflowStepModel) -> Element {
    rsx! {
        li {
            class: CLASS,
            for (index, segment) in props.segments.iter().copied().enumerate() {
                {
                    match segment {
                        HelpSegment::Text { content } => rsx! { "{content}" },
                        HelpSegment::Icon { svg } => rsx! {
                            HelpInlineIcon {
                                key: "{index}",
                                icon: svg,
                            }
                        },
                    }
                }
            }
        }
    }
}

assert_component!(HelpWorkflowStep);
