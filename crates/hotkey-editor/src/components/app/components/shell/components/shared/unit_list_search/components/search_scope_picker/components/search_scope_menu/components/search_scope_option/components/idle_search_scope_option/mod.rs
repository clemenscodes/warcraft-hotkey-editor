mod model;
mod view;

pub use view::IdleSearchScopeOptionView;
mod style;

use dioxus::prelude::*;
use model::IdleSearchScopeOptionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IdleSearchScopeOption(props: IdleSearchScopeOptionModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: false,
            onclick,
            {label}
        }
    }
}

assert_component!(IdleSearchScopeOption);
