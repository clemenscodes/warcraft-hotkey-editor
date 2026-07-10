use super::view::HelpWorkflowStepView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;
use dioxus::prelude::*;

/// The step's input: its segments, rendered in order.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowStepProps {
    pub segments: &'static [HelpSegment],
}

impl From<&HelpWorkflowStepView> for HelpWorkflowStepProps {
    fn from(view: &HelpWorkflowStepView) -> Self {
        let HelpWorkflowStepView { segments } = view.clone();
        Self { segments }
    }
}

impl ddd::Props for HelpWorkflowStepProps {
    type View = HelpWorkflowStepView;
}
