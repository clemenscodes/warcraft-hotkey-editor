mod props;
mod style;

use dioxus::prelude::*;

use style::DIALOG_CLOSE_STYLES;

pub use props::DialogCloseProps;

/// The close glyph in a dialog header. Owns `.dialog-close`; forwards one click.
#[component]
pub fn DialogClose(props: DialogCloseProps) -> Element {
    let onclick = props.onclick;
    rsx! {
        document::Stylesheet { href: DIALOG_CLOSE_STYLES }
        button {
            class: "dialog-close",
            r#type: "button",
            aria_label: "Close",
            onclick,
            "\u{2715}"
        }
    }
}
