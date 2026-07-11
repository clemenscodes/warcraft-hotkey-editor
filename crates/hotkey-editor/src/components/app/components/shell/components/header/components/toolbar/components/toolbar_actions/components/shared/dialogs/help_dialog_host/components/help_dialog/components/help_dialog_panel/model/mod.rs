use super::view::HelpDialogPanelView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpContent;
use dioxus::prelude::*;

/// The help dialog's bordered box inputs: the header title and close handler, the guide
/// content, and the dismiss handler — all plain values it forwards to the header row and
/// the scrolling body.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogPanelModel {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub content: HelpContent,
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpDialogPanelView> for HelpDialogPanelModel {
    fn from(view: &HelpDialogPanelView) -> Self {
        let HelpDialogPanelView {
            title,
            on_close,
            content,
            on_dismiss,
        } = view.clone();
        Self {
            title,
            on_close,
            content,
            on_dismiss,
        }
    }
}

impl ddd::Model for HelpDialogPanelModel {
    type View = HelpDialogPanelView;
}
