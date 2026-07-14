mod model;
mod view;

pub use view::ClearLabelView;
mod style;
use dioxus::prelude::*;
use model::ClearLabelModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ClearLabel(props: ClearLabelModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ClearLabel);
