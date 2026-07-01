pub mod components;
mod hooks;
mod props;

use dioxus::prelude::*;

use super::dialog::Dialog;
use components::template_gallery::TemplateGallery;
use hooks::use_templates_dialog;

pub use props::TemplatesDialogProps;

/// Lets the player apply a bundled layout template. A variant of the `Dialog`
/// base: the hook resolves the templates and their apply handlers, the gallery
/// renders them.
#[component]
pub fn TemplatesDialog(props: TemplatesDialogProps) -> Element {
    let cards = use_templates_dialog(&props);
    let open = props.open;
    if !open() {
        return rsx! {};
    }
    rsx! {
        Dialog {
            open,
            title: "Layout Templates",
            TemplateGallery { cards }
        }
    }
}
