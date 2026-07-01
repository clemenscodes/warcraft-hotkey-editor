use super::components::template_gallery::TemplateGallery;
use super::hooks::TemplatesDialogView;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&TemplatesDialogView> for DialogProps {
    fn from(view: &TemplatesDialogView) -> Self {
        let open = view.open;
        let title = String::from("Layout Templates");
        let gallery = view.gallery.clone();
        let children = rsx! {
            TemplateGallery { ..gallery }
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
