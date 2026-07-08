mod props;
mod style;
use dioxus::prelude::*;
pub use props::EmptyMessageProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(EmptyMessage);
#[component]
pub fn EmptyMessage(props: EmptyMessageProps) -> Element {
    let text = props.text;
    rsx! { p { class: CLASS, {text} } }
}
