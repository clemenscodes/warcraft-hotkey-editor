use dioxus::prelude::*;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_body::components::help_top_row::components::help_workflow_section::HelpWorkflowSectionProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::data::HelpSegment;

/// The callout's input: the workflow steps it frames.
#[derive(Props, Clone, PartialEq)]
pub struct HelpCalloutProps {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpWorkflowSectionProps> for HelpCalloutProps {
    fn from(props: &HelpWorkflowSectionProps) -> Self {
        Self { steps: props.steps }
    }
}
