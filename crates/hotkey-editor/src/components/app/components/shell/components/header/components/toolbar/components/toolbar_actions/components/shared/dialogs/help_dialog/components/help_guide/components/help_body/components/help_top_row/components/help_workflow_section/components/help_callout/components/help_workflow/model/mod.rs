use super::view::HelpWorkflowView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpSegment;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowModel {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpWorkflowView> for HelpWorkflowModel {
    fn from(view: &HelpWorkflowView) -> Self {
        let HelpWorkflowView { steps } = view.clone();
        Self { steps }
    }
}

impl ddd::Model for HelpWorkflowModel {
    type View = HelpWorkflowView;
}
