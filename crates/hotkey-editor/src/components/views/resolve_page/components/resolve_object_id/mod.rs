mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveObjectIdProps;
use style::CLASS;
assert_component!(ResolveObjectId);
#[component]
pub fn ResolveObjectId(props: ResolveObjectIdProps) -> Element {
    let text = props.text;
    rsx! { code { class: CLASS, {text} } }
}
