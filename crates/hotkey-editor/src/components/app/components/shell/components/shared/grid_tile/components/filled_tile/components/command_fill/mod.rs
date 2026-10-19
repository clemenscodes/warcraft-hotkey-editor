mod model;
mod view;

pub use view::CommandFillView;
mod style;

use dioxus::prelude::*;
use model::CommandFillModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CommandFill(props: CommandFillModel) -> Element {
    if !props.active {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
        }
    }
}

assert_component!(CommandFill);
