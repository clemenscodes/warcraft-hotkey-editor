use super::view::HelpTopRowView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_guide_host::data::HelpContent;
use dioxus::prelude::*;

/// The top row's input: the content, split between its two columns.
#[derive(Props, Clone, PartialEq)]
pub struct HelpTopRowModel {
    pub content: HelpContent,
}

impl From<&HelpTopRowView> for HelpTopRowModel {
    fn from(view: &HelpTopRowView) -> Self {
        let HelpTopRowView { content } = view.clone();
        Self { content }
    }
}

impl ddd::Model for HelpTopRowModel {
    type View = HelpTopRowView;
}
