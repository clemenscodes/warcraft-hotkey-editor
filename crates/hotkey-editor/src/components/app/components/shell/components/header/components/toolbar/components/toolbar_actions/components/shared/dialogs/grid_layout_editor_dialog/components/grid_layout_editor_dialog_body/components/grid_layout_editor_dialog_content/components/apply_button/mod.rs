mod data;
mod model;
mod view;

pub use view::ApplyButtonView;
mod style;

use dioxus::prelude::*;
use model::ApplyButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ApplyButton(props: ApplyButtonModel) -> Element {
    let onclick = props.on_apply;
    let label = data::LABEL;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
        }
    }
}

assert_component!(ApplyButton);
