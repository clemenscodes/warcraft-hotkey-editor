mod model;
mod view;

pub use view::DialogCloseView;
mod style;

use crate::components::app::components::shell::components::shared::icons::ICON_CLOSE;
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
            dangerous_inner_html: ICON_CLOSE,
        }
    }
}

assert_component!(DialogClose);
