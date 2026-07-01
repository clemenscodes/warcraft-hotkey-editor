pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::assert_component;
use components::dialog_body::{DialogBody, DialogBodyProps};
use components::dialog_footer::{DialogFooter, DialogFooterProps};
use components::dialog_header::{DialogHeader, DialogHeaderProps};
use hooks::use_body_scroll_lock;
use props::DialogChrome;
use style::{CLASS, OVERLAY};

pub use props::DialogProps;

assert_component!(Dialog);

/// The one dialog: a dimmed backdrop centring a bordered box that holds a header,
/// a scrolling body, and an optional footer. Every concrete dialog composes this
/// with its open signal, title, and body — there is no other dialog shell.
#[component]
pub fn Dialog(props: DialogProps) -> Element {
    use_body_scroll_lock(props.open);
    let DialogChrome {
        open,
        on_open_change,
    } = DialogChrome::from(&props);
    let header = DialogHeaderProps::from(&props);
    let body = DialogBodyProps::from(&props);
    let footer = DialogFooterProps::from(&props);
    rsx! {
        DialogRoot {
            class: OVERLAY,
            open,
            on_open_change,
            DialogContent {
                class: CLASS.to_library_class(),
                DialogHeader { ..header }
                DialogBody { ..body }
                DialogFooter { ..footer }
            }
        }
    }
}
