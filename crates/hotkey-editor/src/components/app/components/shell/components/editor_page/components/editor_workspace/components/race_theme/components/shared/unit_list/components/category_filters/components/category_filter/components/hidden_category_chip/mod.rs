mod model;
mod view;

pub use view::HiddenCategoryChipView;
mod style;

use dioxus::prelude::*;
use model::HiddenCategoryChipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HiddenCategoryChip(props: HiddenCategoryChipModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: false,
            onclick,
            {label}
        }
    }
}

assert_component!(HiddenCategoryChip);
