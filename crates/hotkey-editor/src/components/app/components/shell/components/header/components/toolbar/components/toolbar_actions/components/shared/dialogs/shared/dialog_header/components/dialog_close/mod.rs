mod props;
mod style;

use dioxus::prelude::*;
pub use props::DialogCloseProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(DialogClose);
