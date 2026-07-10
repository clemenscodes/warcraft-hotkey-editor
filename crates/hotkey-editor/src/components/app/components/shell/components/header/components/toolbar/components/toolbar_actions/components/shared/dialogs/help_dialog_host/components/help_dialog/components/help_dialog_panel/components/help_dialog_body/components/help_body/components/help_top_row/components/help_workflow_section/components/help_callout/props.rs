use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;
use dioxus::prelude::*;

/// The callout's input: the workflow steps it frames.
#[derive(Props, Clone, PartialEq)]
pub struct HelpCalloutProps {
    pub steps: &'static [&'static [HelpSegment]],
}
