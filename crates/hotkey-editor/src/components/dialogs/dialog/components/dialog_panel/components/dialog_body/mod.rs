mod props;
mod style;

use dioxus::prelude::*;

use style::DIALOG_BODY_STYLE_SHEETS;

pub use props::DialogBodyProps;

/// The dialog's scrolling content region between the header and the panel edge.
/// Owns `.dialog-body`, including its gold scrollbar.
#[component]
pub fn DialogBody(props: DialogBodyProps) -> Element {
    let body = props.children.clone();
    rsx! {
        for href in DIALOG_BODY_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "dialog-body",
            {body}
        }
    }
}
