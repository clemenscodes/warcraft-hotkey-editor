mod props;
mod view;

pub use view::ConflictCardCaptionView;
mod style;
use dioxus::prelude::*;
use props::ConflictCardCaptionProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictCardCaption(props: ConflictCardCaptionProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}

assert_component!(ConflictCardCaption);
