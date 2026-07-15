mod model;
mod view;

pub use view::ShownCategoryChipView;
mod style;

use dioxus::prelude::*;
use model::ShownCategoryChipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ShownCategoryChip(props: ShownCategoryChipModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_pressed: true,
            onclick,
            {label}
        }
    }
}

assert_component!(ShownCategoryChip);
