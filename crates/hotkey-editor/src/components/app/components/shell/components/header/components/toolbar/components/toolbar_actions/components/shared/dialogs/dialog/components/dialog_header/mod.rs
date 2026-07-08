pub mod components;
mod props;
mod style;

use components::dialog_close::{DialogClose, DialogCloseProps};
use components::dialog_header_decoration_leading::DialogHeaderDecorationLeading;
use components::dialog_header_decoration_trailing::DialogHeaderDecorationTrailing;
use components::dialog_title::DialogTitle;
use dioxus::prelude::*;
pub use props::DialogHeaderProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DialogHeader);

/// A dialog's title bar: mirrored gold decorations either side of the title,
/// with the close control pinned right.
#[component]
pub fn DialogHeader(props: DialogHeaderProps) -> Element {
    let title = props.title.clone();
    let close = DialogCloseProps::from(&props);
    rsx! {
        header { class: CLASS,
            DialogHeaderDecorationLeading {}
            DialogTitle { title }
            DialogHeaderDecorationTrailing {}
            DialogClose { ..close }
        }
    }
}
