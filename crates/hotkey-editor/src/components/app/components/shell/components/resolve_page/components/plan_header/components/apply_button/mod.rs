mod data;
mod props;
mod style;
use dioxus::prelude::*;
use props::ApplyButtonProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ApplyButton(props: ApplyButtonProps) -> Element {
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
