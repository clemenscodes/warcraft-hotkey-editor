use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpContent;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`HelpDialogPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HelpDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub content: HelpContent,
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl ddd::View for HelpDialogPanelView {}
