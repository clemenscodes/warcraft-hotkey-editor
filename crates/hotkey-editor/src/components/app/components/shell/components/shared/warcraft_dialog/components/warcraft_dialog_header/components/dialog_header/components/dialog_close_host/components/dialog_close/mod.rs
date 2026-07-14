mod model;
mod view;

pub use view::DialogCloseView;
mod style;

use dioxus::prelude::*;
use model::DialogCloseModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogClose(props: DialogCloseModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Close",
            onclick,
            "\u{2715}"
        }
    }
}

assert_component!(DialogClose);
