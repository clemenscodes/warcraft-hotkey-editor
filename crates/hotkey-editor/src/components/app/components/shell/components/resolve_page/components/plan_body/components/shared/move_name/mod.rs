mod props;
mod style;
use dioxus::prelude::*;
pub use props::MoveNameProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveName);
#[component]
pub fn MoveName(props: MoveNameProps) -> Element {
    let text = props.text;
    let is_link = props.is_link;
    rsx! { span { class: CLASS, "data-link": is_link, {text} } }
}
