mod model;
mod view;

pub use view::ConflictCardCaptionView;
mod style;
use dioxus::prelude::*;
use model::ConflictCardCaptionModel;
use style::CLASS;
use tw_macro::assert_component;
#[component]
pub fn ConflictCardCaption(props: ConflictCardCaptionModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ConflictCardCaption);
