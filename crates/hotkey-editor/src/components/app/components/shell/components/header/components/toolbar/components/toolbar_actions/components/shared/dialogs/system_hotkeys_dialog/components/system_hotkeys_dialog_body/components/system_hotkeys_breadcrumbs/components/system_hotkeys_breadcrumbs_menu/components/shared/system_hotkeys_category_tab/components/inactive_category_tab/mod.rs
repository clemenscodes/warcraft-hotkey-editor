mod model;
mod view;

pub use view::InactiveCategoryTabView;
mod style;

use dioxus::prelude::*;
use model::InactiveCategoryTabModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InactiveCategoryTab(props: InactiveCategoryTabModel) -> Element {
    let label = props.label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: false,
            aria_current: "false",
            onclick: on_click,
            {label}
        }
    }
}

assert_component!(InactiveCategoryTab);
