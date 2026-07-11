use super::view::HelpDialogBodyView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpContent;
use dioxus::prelude::*;

/// The help dialog's scroll region inputs: the guide content it lays out and the dismiss
/// handler for the button beneath it.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogBodyModel {
    pub content: HelpContent,
    pub on_dismiss: EventHandler<MouseEvent>,
}

impl From<&HelpDialogBodyView> for HelpDialogBodyModel {
    fn from(view: &HelpDialogBodyView) -> Self {
        let HelpDialogBodyView {
            content,
            on_dismiss,
        } = view.clone();
        Self {
            content,
            on_dismiss,
        }
    }
}

impl ddd::Model for HelpDialogBodyModel {
    type View = HelpDialogBodyView;
}
