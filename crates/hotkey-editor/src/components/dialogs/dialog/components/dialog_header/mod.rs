pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use components::dialog_close::{DialogClose, DialogCloseProps};
use components::dialog_header_decoration_leading::DialogHeaderDecorationLeading;
use components::dialog_header_decoration_trailing::DialogHeaderDecorationTrailing;
use components::dialog_title::DialogTitle;
use style::CLASS;

pub use props::DialogHeaderProps;

assert_component!(DialogHeader);

/// A dialog's title bar: mirrored gold decorations either side of the title,
/// with the close control pinned right.
#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    let title = props.title.clone();
    let close = DialogCloseProps::from(&props);
    rsx! {
        header {
            class: CLASS,
            DialogHeaderDecorationLeading {}
            DialogTitle { {title} }
            DialogHeaderDecorationTrailing {}
            DialogClose { ..close }
        }
    }
}
