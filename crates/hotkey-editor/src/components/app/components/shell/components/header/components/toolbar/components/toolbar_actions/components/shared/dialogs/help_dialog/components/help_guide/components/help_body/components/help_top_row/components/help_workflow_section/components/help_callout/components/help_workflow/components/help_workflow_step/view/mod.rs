use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpSegment;

/// The published `View` contract mirroring [`HelpWorkflowStepModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpWorkflowStepView {
    pub segments: &'static [HelpSegment],
}

impl ddd::View for HelpWorkflowStepView {}
