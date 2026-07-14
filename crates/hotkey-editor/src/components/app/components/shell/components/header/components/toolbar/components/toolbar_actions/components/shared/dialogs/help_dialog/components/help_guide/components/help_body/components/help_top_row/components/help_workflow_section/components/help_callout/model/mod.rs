use super::view::HelpCalloutView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpSegment;
use dioxus::prelude::*;

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
