use super::components::preview_textarea::PreviewTextarea;
use super::hooks::PreviewDialogView;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&PreviewDialogView> for DialogProps {
    fn from(view: &PreviewDialogView) -> Self {
        let open = view.open;
        let title = String::from("Preview");
        let textarea = view.textarea.clone();
        let children = rsx! {
            PreviewTextarea { ..textarea }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
