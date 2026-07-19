mod model;
mod view;

pub use view::ResolveSectionTabCountView;
mod style;

use dioxus::prelude::*;
use model::ResolveSectionTabCountModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolveSectionTabCount(props: ResolveSectionTabCountModel) -> Element {
    let count = props.count;
    rsx! {
        span {
            class: CLASS,
            "{count}"
        }
    }
}

assert_component!(ResolveSectionTabCount);
