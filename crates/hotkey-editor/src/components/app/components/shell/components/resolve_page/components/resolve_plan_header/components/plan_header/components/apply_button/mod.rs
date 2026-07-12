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
    let running = props.running;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled: running,
            onclick,
            if running {
                {data::APPLYING}
            } else {
                {data::APPLY}
            }
        }
    }
}

assert_component!(ApplyButton);
