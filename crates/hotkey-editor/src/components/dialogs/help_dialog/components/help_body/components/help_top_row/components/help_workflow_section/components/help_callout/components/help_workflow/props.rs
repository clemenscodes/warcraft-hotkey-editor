use dioxus::prelude::*;

use crate::components::dialogs::help_dialog::components::help_body::components::help_top_row::components::help_workflow_section::components::help_callout::HelpCalloutProps;
use crate::components::dialogs::help_dialog::data::HelpSegment;

/// The workflow list's input: the steps to render, each its own segment list.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowProps {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpCalloutProps> for HelpWorkflowProps {
    fn from(props: &HelpCalloutProps) -> Self {
        Self { steps: props.steps }
    }
}
