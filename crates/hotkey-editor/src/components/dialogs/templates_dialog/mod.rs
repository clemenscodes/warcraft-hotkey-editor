pub mod components;
mod hooks;
mod logic;
mod props;

use super::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_templates_dialog;
pub use props::TemplatesDialogProps;

/// Lets the player apply a bundled layout template. A variant of the `Dialog`
/// base: the hook resolves the templates and their apply handlers, the gallery
/// renders them.
#[component]
pub fn TemplatesDialog(props: TemplatesDialogProps) -> Element {
    let view = use_templates_dialog(&props);
    let open = view.open;
    if !open() {
        return rsx! {};
    }
    rsx! {
        Dialog { ..DialogProps::from(&view) }
    }
}
