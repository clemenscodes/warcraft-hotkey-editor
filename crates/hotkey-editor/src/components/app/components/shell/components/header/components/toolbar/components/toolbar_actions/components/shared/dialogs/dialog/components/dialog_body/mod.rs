mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::DialogBodyProps;
use style::CLASS;
assert_component!(DialogBody);

/// The dialog's scrolling content region between the header and the panel edge,
/// including its gold scrollbar.
#[component]
pub fn DialogBody(props: DialogBodyProps) -> Element {
    let body = props.children.clone();
    rsx! {
        div { class: CLASS, {body} }
    }
}
