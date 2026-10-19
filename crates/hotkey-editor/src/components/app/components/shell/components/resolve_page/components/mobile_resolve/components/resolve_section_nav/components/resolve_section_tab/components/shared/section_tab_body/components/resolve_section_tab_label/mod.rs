mod model;
mod view;

pub use view::ResolveSectionTabLabelView;
mod style;

use dioxus::prelude::*;
use model::ResolveSectionTabLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ResolveSectionTabLabel(props: ResolveSectionTabLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ResolveSectionTabLabel);
