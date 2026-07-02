mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ConflictCardCaptionProps;
use style::CLASS;
assert_component!(ConflictCardCaption);
#[component]
pub fn ConflictCardCaption(props: ConflictCardCaptionProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
