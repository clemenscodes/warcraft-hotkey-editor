mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ApplyButtonProps;
use style::CLASS;
assert_component!(ApplyButton);
#[component]
pub fn ApplyButton(props: ApplyButtonProps) -> Element {
    let running = props.running;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled: running,
            "data-action": "apply-cascade",
            onclick,
            if running {
                "Applying…"
            } else {
                "Apply"
            }
        }
    }
}
