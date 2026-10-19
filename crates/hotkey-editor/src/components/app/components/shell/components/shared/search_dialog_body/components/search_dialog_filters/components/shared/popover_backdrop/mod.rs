mod model;
mod view;

pub use view::PopoverBackdropView;
mod style;

use dioxus::prelude::*;
use model::PopoverBackdropModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PopoverBackdrop(props: PopoverBackdropModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            onclick,
        }
    }
}

assert_component!(PopoverBackdrop);
