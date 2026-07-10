use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::components::help_dialog_panel::components::help_dialog_body::components::help_body::components::help_top_row::HelpTopRowProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;
use dioxus::prelude::*;

/// The workflow column's input: the steps, each a sequence of text/icon segments.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowSectionProps {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpTopRowProps> for HelpWorkflowSectionProps {
    fn from(props: &HelpTopRowProps) -> Self {
        Self {
            steps: props.content.workflow(),
        }
    }
}
