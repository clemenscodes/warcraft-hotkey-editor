mod props;
mod style;
use dioxus::prelude::*;
pub use props::UnresolvedTitleProps;
use style::CLASS;
use tw_macro::assert_component;
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
