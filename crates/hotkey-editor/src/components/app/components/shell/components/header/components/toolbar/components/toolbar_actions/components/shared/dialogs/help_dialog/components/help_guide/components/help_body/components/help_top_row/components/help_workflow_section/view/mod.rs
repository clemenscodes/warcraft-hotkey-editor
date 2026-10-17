use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpSegment;

#[derive(Clone, PartialEq)]
pub struct HelpWorkflowSectionView {
    pub steps: &'static [&'static [HelpSegment]],
}

impl ddd::Layered for HelpWorkflowSectionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpWorkflowSectionView {}
