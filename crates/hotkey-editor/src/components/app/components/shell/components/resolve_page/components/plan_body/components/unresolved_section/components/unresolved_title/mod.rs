mod model;
mod view;

pub use view::UnresolvedTitleView;
mod style;
use dioxus::prelude::*;
use model::UnresolvedTitleModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn UnresolvedTitle(props: UnresolvedTitleModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(UnresolvedTitle);
