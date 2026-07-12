use super::view::HelpWorkflowStepView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpSegment;
use dioxus::prelude::*;

/// The step's input: its segments, rendered in order.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowStepModel {
    pub segments: &'static [HelpSegment],
}

impl From<&HelpWorkflowStepView> for HelpWorkflowStepModel {
    fn from(view: &HelpWorkflowStepView) -> Self {
        let HelpWorkflowStepView { segments } = view.clone();
        Self { segments }
    }
}

impl ddd::Model for HelpWorkflowStepModel {
    type View = HelpWorkflowStepView;
}
