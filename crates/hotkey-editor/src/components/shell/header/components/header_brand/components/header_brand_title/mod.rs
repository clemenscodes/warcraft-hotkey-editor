mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HeaderBrandTitleProps;

assert_component!(HeaderBrandTitle);

#[component]
pub fn HeaderBrandTitle(props: HeaderBrandTitleProps) -> Element {
    let title = props.title;
    rsx! {
        h1 {
            class: CLASS,
            {title}
        }
    }
}
