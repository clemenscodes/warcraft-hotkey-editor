mod props;
mod style;
use dioxus::prelude::*;
pub use props::EmptyMessageProps;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn EmptyMessage(props: EmptyMessageProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}

assert_component!(EmptyMessage);
