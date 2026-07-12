use super::view::HelpCalloutView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpSegment;
use dioxus::prelude::*;

/// The callout's input: the workflow steps it frames.
#[derive(Props, Clone, PartialEq)]
pub struct HelpCalloutModel {
    pub steps: &'static [&'static [HelpSegment]],
}

impl From<&HelpCalloutView> for HelpCalloutModel {
    fn from(view: &HelpCalloutView) -> Self {
        let HelpCalloutView { steps } = view.clone();
        Self { steps }
    }
}

impl ddd::Model for HelpCalloutModel {
    type View = HelpCalloutView;
}
