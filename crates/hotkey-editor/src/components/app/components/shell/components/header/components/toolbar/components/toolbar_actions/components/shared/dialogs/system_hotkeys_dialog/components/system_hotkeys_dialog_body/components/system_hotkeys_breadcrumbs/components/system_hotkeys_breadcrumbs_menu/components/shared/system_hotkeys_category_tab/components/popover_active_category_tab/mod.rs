mod model;
mod view;

pub use view::PopoverActiveCategoryTabView;
mod style;

use dioxus::prelude::*;
use model::PopoverActiveCategoryTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PopoverActiveCategoryTab(props: PopoverActiveCategoryTabModel) -> Element {
    let label = props.label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: true,
            aria_current: "page",
            onclick: on_click,
            {label}
        }
    }
}

assert_component!(PopoverActiveCategoryTab);
