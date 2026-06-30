use dioxus::prelude::*;

use crate::components::dialogs::help_dialog::content::HelpSegment;

/// The step's input: its segments, rendered in order.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowStepProps {
    pub segments: &'static [HelpSegment],
}
