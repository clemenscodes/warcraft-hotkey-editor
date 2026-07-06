mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictCardCaptionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictCardCaption);
#[component]
pub fn ConflictCardCaption(props: ConflictCardCaptionProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
