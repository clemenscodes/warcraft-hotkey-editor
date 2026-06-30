pub mod components;
mod props;
mod style;

use dioxus::prelude::*;
use dioxus_primitives::dialog::DialogContent;

use crate::assert_component;
use components::dialog_body::{DialogBody, DialogBodyProps};
use components::dialog_footer::{DialogFooter, DialogFooterProps};
use components::dialog_header::{DialogHeader, DialogHeaderProps};
use style::CLASS;

pub use props::DialogPanelProps;

assert_component!(DialogPanel);

/// The dialog panel: the bordered, sized box that holds the header above the
/// scrolling body. Owns `.dialog-panel` on the library `DialogContent` (the
/// role=dialog element), which takes a `String` class, hence the bridge.
#[component]
pub fn DialogPanel(props: DialogPanelProps) -> Element {
    let header = DialogHeaderProps::from(&props);
    let body = DialogBodyProps::from(&props);
    let footer = DialogFooterProps::from(&props);
    rsx! {
        DialogContent {
            class: CLASS.to_library_class(),
            DialogHeader { ..header }
            DialogBody { ..body }
            DialogFooter { ..footer }
        }
    }
}
