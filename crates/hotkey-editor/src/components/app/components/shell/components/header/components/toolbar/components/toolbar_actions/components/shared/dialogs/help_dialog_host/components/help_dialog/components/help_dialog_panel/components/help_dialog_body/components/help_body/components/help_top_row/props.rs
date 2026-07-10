use super::view::HelpTopRowView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpContent;
use dioxus::prelude::*;

/// The top row's input: the content, split between its two columns.
#[derive(Props, Clone, PartialEq)]
pub struct HelpTopRowProps {
    pub content: HelpContent,
}

impl From<&HelpTopRowView> for HelpTopRowProps {
    fn from(view: &HelpTopRowView) -> Self {
        let HelpTopRowView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Props for HelpTopRowProps {
    type View = HelpTopRowView;
}
