use super::view::HelpWorkflowSectionView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpSegment;
use dioxus::prelude::*;

/// The workflow column's input: the steps, each a sequence of text/icon segments.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowSectionModel {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpWorkflowSectionView> for HelpWorkflowSectionModel {
    fn from(view: &HelpWorkflowSectionView) -> Self {
        let HelpWorkflowSectionView { steps } = view.clone();
        Self { steps }
    }
}

impl ddd::Model for HelpWorkflowSectionModel {
    type View = HelpWorkflowSectionView;
}
