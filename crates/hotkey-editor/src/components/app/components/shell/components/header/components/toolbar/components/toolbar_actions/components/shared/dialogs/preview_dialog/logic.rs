use super::components::preview_textarea::PreviewTextarea;
use super::hooks::PreviewDialogView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
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
            on_open_change: None,
        }
    }
}
