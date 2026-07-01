mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CollisionsEmptyMessageProps;
use style::CLASS;
assert_component!(CollisionsEmptyMessage);
#[component]
pub fn CollisionsEmptyMessage(props: CollisionsEmptyMessageProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}
