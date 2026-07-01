mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::DialogCloseProps;
use style::CLASS;
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
