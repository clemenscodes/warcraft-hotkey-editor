mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveMoveNameProps;
use style::CLASS;
assert_component!(ResolveMoveName);
#[component]
pub fn ResolveMoveName(props: ResolveMoveNameProps) -> Element {
    let text = props.text;
    let is_link = props.is_link;
    rsx! { span { class: CLASS, "data-link": is_link, {text} } }
}
