mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::ResolveApplyButtonProps;
use style::CLASS;
assert_component!(ResolveApplyButton);
#[component]
pub fn ResolveApplyButton(props: ResolveApplyButtonProps) -> Element {
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
