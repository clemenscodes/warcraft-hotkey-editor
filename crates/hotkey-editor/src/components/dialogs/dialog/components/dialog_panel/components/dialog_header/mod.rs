pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::dialog_close::{DialogClose, DialogCloseProps};
use components::dialog_header_decoration::DialogHeaderDecoration;
use components::dialog_title::DialogTitle;
use style::DIALOG_HEADER_STYLE_SHEETS;

pub use props::DialogHeaderProps;

/// A dialog's title bar: mirrored gold decorations either side of the title,
/// with the close control pinned right. Owns `.dialog-header`; the title,
/// decorations, and close are children.
#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    let title = props.title.clone();
    let close = DialogCloseProps::from(&props);
    rsx! {
        for href in DIALOG_HEADER_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        header {
            class: "dialog-header",
            DialogHeaderDecoration { flipped: false }
            DialogTitle { {title} }
            DialogHeaderDecoration { flipped: true }
            DialogClose { ..close }
        }
    }
}
