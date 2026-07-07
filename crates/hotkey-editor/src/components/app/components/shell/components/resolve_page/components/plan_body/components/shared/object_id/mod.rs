mod props;
mod style;
use dioxus::prelude::*;
pub use props::ObjectIdProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ObjectId);
#[component]
pub fn ObjectId(props: ObjectIdProps) -> Element {
    let text = props.text;
    rsx! { code { class: CLASS, {text} } }
}
