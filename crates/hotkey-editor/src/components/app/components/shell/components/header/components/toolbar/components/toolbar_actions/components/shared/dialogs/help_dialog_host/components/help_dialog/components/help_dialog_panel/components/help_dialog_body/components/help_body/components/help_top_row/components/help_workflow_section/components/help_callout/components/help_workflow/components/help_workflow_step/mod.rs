pub mod components;
mod model;
mod view;

pub use view::HelpWorkflowStepView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::help_top_row::components::help_workflow_section::components::help_callout::components::help_workflow::components::help_workflow_step::components::help_inline_icon::HelpInlineIcon;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;
use dioxus::prelude::*;
use model::HelpWorkflowStepModel;
use style::CLASS;
use tw_macro::assert_component;

/// One step in the workflow list. A pure loop over its segments: a text run is
/// printed, an icon segment renders an inline glyph.
#[component]
pub fn HelpWorkflowStep(props: HelpWorkflowStepModel) -> Element {
    rsx! {
        li { class: CLASS,
            for (index, segment) in props.segments.iter().copied().enumerate() {
                {
                    match segment {
                        HelpSegment::Text { content } => rsx! { "{content}" },
                        HelpSegment::Icon { svg } => rsx! {
                            HelpInlineIcon { key: "{index}", icon: svg }
                        },
                    }
                }
            }
        }
    }
}

assert_component!(HelpWorkflowStep);
