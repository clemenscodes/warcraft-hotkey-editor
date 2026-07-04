mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::BrandTitleProps;
use style::CLASS;
assert_component!(BrandTitle);

#[component]
pub fn BrandTitle(props: BrandTitleProps) -> Element {
    let title = props.title;
    rsx! {
        h1 { class: CLASS, {title} }
    }
}
