pub mod components;
mod model;
mod view;

pub use view::DialogHeaderView;
mod style;

use components::dialog_close::DialogClose;
use components::dialog_header_decoration_leading::DialogHeaderDecorationLeading;
use components::dialog_header_decoration_trailing::DialogHeaderDecorationTrailing;
use components::dialog_title::DialogTitle;
use dioxus::prelude::*;
use model::DialogHeaderModel;
use style::CLASS;
use tw_macro::assert_component;

/// A dialog's title bar: mirrored gold decorations either side of the title,
/// with the close control pinned right.
#[component]
pub fn DialogHeader(props: DialogHeaderModel) -> Element {
    let title = props.title.clone();
    let on_close = props.on_close;
    let onclick = EventHandler::new(move |_event: MouseEvent| on_close.call(()));
    rsx! {
        header { class: CLASS,
            DialogHeaderDecorationLeading {}
            DialogTitle { title }
            DialogHeaderDecorationTrailing {}
            DialogClose { onclick }
        }
    }
}

assert_component!(DialogHeader);
