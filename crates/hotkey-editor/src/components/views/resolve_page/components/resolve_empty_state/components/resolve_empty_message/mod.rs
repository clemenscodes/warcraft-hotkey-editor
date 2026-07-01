mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveEmptyMessageProps;
use style::CLASS;
assert_component!(ResolveEmptyMessage);
#[component]
pub fn ResolveEmptyMessage(props: ResolveEmptyMessageProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}
