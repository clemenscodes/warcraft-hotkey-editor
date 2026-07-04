mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::EmptyMessageProps;
use style::CLASS;
assert_component!(EmptyMessage);
#[component]
pub fn EmptyMessage(props: EmptyMessageProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}
