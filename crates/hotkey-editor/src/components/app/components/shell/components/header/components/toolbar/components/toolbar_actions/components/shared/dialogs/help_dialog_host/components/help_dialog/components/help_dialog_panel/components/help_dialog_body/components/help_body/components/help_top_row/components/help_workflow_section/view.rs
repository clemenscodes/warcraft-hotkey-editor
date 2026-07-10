use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;

/// The published `View` contract mirroring [`HelpWorkflowSectionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpWorkflowSectionView {
    pub steps: &'static [&'static [HelpSegment]],
}

impl ddd::View for HelpWorkflowSectionView {}
