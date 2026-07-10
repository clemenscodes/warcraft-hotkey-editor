mod props;
mod view;

pub use view::BrandTitleView;
mod style;

use dioxus::prelude::*;
use props::BrandTitleProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BrandTitle(props: BrandTitleProps) -> Element {
    let title = props.title;
    rsx! {
        h1 { class: CLASS, {title} }
    }
}

assert_component!(BrandTitle);
