mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::DialogBodyProps;

assert_component!(DialogBody);

/// The dialog's scrolling content region between the header and the panel edge,
/// including its gold scrollbar.
#[component]
pub fn DialogBody(props: DialogBodyProps) -> Element {
    let body = props.children.clone();
    rsx! {
        div {
            class: CLASS,
            {body}
        }
    }
}
