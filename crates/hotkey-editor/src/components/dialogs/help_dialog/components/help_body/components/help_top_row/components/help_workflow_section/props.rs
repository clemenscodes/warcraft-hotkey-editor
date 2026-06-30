use dioxus::prelude::*;

use crate::components::dialogs::help_dialog::components::help_body::components::help_top_row::HelpTopRowProps;
use crate::components::dialogs::help_dialog::content::HelpSegment;

/// The workflow column's input: the steps, each a sequence of text/icon segments.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowSectionProps {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpTopRowProps> for HelpWorkflowSectionProps {
    fn from(props: &HelpTopRowProps) -> Self {
        Self {
            steps: props.content.workflow,
        }
    }
}
