mod model;
mod view;

pub use view::ActiveCategoryTabView;
mod style;

use dioxus::prelude::*;
use model::ActiveCategoryTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveCategoryTab(props: ActiveCategoryTabModel) -> Element {
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

assert_component!(ActiveCategoryTab);
