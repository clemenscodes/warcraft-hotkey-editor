pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogRoot;

use components::dialog_panel::{DialogPanel, DialogPanelProps};
use hooks::use_body_scroll_lock;
use props::DialogChrome;
use style::DIALOG_STYLE_SHEETS;

pub use props::DialogProps;

/// The dialog backdrop. Owns `.dialog`, locks body scroll while open, and hands
/// the panel everything below. Every concrete dialog is a variant that hands
/// this its open signal, title, and body.
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    use_body_scroll_lock(props.open);
    let DialogChrome {
        open,
        on_open_change,
    } = DialogChrome::from(&props);
    let panel = DialogPanelProps::from(&props);
    rsx! {
        for href in DIALOG_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        DialogRoot {
            class: "dialog",
            open,
            on_open_change,
            DialogPanel { ..panel }
        }
    }
}
