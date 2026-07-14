use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpSegment;

#[derive(Clone, PartialEq)]
pub struct HelpWorkflowView {
    pub steps: &'static [&'static [HelpSegment]],
}

impl ddd::View for HelpWorkflowView {}
