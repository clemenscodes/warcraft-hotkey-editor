mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HeaderBrandTitleProps;
use style::CLASS;
assert_component!(HeaderBrandTitle);

#[component]
pub fn HeaderBrandTitle(props: HeaderBrandTitleProps) -> Element {
    let title = props.title;
    rsx! {
        h1 { class: CLASS, {title} }
    }
}
