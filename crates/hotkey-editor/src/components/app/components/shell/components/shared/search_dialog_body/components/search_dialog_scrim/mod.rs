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
    let leaving = props.leaving;
    rsx! {
        div {
            class: CLASS,
            role: "button",
            aria_label: "Close filters",
            aria_hidden: leaving,
            tabindex: "-1",
            onclick,
        }
    }
}

assert_component!(SearchDialogScrim);
