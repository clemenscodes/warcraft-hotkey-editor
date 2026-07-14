mod model;
mod view;

pub use view::EmptyMessageView;
mod style;
use dioxus::prelude::*;
use model::EmptyMessageModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn EmptyMessage(props: EmptyMessageModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(EmptyMessage);
