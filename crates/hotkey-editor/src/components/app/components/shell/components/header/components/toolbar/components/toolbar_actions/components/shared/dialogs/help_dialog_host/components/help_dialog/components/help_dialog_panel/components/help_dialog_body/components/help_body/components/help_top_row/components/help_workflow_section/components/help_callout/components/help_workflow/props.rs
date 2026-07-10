use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpSegment;
use dioxus::prelude::*;

/// The workflow list's input: the steps to render, each its own segment list.
#[derive(Props, Clone, PartialEq)]
pub struct HelpWorkflowProps {
    pub steps: &'static [&'static [HelpSegment]],
}
