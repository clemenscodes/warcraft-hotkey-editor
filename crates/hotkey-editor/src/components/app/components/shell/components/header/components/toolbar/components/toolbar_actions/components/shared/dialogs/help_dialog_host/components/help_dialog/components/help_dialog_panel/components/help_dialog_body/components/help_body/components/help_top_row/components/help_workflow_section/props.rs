use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;
use dioxus::prelude::*;

/// The workflow column's input: the steps, each a sequence of text/icon segments.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowSectionProps {
    pub steps: &'static [&'static [HelpSegment]],
}
