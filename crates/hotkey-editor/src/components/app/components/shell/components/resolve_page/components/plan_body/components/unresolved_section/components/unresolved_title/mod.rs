mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::UnresolvedTitleProps;
use style::CLASS;
assert_component!(UnresolvedTitle);
#[component]
pub fn UnresolvedTitle(props: UnresolvedTitleProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
