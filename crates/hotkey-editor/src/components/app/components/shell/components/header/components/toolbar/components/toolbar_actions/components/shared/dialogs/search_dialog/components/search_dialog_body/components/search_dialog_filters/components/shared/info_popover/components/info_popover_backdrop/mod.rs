mod model;
mod view;

pub use view::InfoPopoverBackdropView;
mod style;

use dioxus::prelude::*;
use model::InfoPopoverBackdropModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoPopoverBackdrop(props: InfoPopoverBackdropModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            onclick,
        }
    }
}

assert_component!(InfoPopoverBackdrop);
