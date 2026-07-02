mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ObjectIdProps;
use style::CLASS;
assert_component!(ObjectId);
#[component]
pub fn ObjectId(props: ObjectIdProps) -> Element {
    let text = props.text;
    rsx! { code { class: CLASS, {text} } }
}
