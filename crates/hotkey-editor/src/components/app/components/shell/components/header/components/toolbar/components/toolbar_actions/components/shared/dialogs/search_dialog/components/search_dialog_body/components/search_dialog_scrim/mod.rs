mod model;
mod view;

pub use view::SearchDialogScrimView;
mod style;

use dioxus::prelude::*;
use model::SearchDialogScrimModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchDialogScrim(props: SearchDialogScrimModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            role: "button",
            aria_label: "Close filters",
            tabindex: "-1",
            onclick,
        }
    }
}

assert_component!(SearchDialogScrim);
