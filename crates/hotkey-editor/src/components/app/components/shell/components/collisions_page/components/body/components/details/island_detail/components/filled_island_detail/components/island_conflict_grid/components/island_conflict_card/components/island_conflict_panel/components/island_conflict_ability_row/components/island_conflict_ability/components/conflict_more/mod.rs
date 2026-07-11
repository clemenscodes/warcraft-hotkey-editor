mod data;
mod model;
mod view;

pub use view::ConflictMoreView;
mod style;
use dioxus::prelude::*;
use model::ConflictMoreModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictMore(props: ConflictMoreModel) -> Element {
    let count = props.count;
    let onclick = props.onclick;
    let more_label = data::MORE_LABEL;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            "+{count} {more_label}"
        }
    }
}

assert_component!(ConflictMore);
