mod props;
mod style;
use dioxus::prelude::*;
use props::ObjectIdProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ObjectId(props: ObjectIdProps) -> Element {
    let text = props.text;
    rsx! { code { class: CLASS, {text} } }
}

assert_component!(ObjectId);
