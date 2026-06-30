mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::DialogCloseProps;

assert_component!(DialogClose);

/// The close glyph in a dialog header; forwards one click.
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Close",
            onclick,
            "\u{2715}"
        }
    }
}
