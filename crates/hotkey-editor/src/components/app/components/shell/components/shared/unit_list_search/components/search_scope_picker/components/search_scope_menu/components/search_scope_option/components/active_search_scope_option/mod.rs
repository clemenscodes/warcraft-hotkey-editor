mod model;
mod view;

pub use view::ActiveSearchScopeOptionView;
mod style;

use dioxus::prelude::*;
use model::ActiveSearchScopeOptionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveSearchScopeOption(props: ActiveSearchScopeOptionModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            role: "option",
            aria_selected: true,
            onclick,
            {label}
        }
    }
}

assert_component!(ActiveSearchScopeOption);
