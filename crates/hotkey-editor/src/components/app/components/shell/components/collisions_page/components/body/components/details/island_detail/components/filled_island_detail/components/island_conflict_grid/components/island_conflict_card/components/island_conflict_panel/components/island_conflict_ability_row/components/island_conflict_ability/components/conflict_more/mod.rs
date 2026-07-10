mod data;
mod props;
mod style;
use dioxus::prelude::*;
pub use props::ConflictMoreProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictMore(props: ConflictMoreProps) -> Element {
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
