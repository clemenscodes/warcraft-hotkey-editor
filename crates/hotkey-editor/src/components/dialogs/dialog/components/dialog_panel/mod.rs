pub mod components;
mod props;
mod style;

use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;

use components::dialog_body::{DialogBody, DialogBodyProps};
use components::dialog_header::{DialogHeader, DialogHeaderProps};
use style::DIALOG_PANEL_STYLE_SHEETS;

pub use props::DialogPanelProps;

/// The dialog panel: the bordered, sized box that holds the header above the
/// scrolling body. Owns `.dialog-panel`. The header and body are children built
/// by `From`.
#[component]
pub fn DialogPanel(props: DialogPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = DialogBodyProps::from(&props);
    let panel_class = props.panel_class.clone();
    rsx! {
        for href in DIALOG_PANEL_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        DialogContent {
            class: panel_class,
            DialogHeader { ..header }
            DialogBody { ..body }
        }
    }
}
