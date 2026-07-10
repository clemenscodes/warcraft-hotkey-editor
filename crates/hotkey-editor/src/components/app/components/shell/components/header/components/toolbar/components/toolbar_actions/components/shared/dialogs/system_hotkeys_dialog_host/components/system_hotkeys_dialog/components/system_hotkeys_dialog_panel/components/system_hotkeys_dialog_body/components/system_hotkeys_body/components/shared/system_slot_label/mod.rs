mod props;
mod style;

use dioxus::prelude::*;
pub use props::SystemSlotLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemSlotLabel);

/// The caption shown above the key on a system hotkey slot.
#[component]
pub fn SystemSlotLabel(props: SystemSlotLabelProps) -> Element {
    let text = props.text;
    rsx! {
        div {
            class: CLASS,
            {text}
        }
    }
}
