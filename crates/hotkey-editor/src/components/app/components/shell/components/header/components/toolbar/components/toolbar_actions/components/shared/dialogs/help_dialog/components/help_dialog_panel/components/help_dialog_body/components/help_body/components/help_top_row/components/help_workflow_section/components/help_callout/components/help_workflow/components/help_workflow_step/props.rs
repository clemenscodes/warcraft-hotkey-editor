use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::data::HelpSegment;
use dioxus::prelude::*;

/// The step's input: its segments, rendered in order.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowStepProps {
    pub segments: &'static [HelpSegment],
}
