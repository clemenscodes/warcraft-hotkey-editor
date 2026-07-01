mod logic;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use logic::SystemSlotLabelPresentation;
use style::CLASS;

pub use props::SystemSlotLabelProps;

assert_component!(SystemSlotLabel);

/// The caption shown above the key on a system hotkey slot.
#[component]
pub fn SystemSlotLabel(props: SystemSlotLabelProps) -> Element {
    let SystemSlotLabelPresentation { text, compact } = SystemSlotLabelPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            "data-compact": compact,
            {text}
        }
    }
}
