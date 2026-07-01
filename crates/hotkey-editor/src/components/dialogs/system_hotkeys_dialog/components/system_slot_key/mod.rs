mod logic;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use logic::SystemSlotKeyPresentation;
use style::CLASS;

pub use props::SystemSlotKeyProps;

assert_component!(SystemSlotKey);

/// The bound-key glyph shown on a system hotkey slot.
#[component]
pub fn SystemSlotKey(props: SystemSlotKeyProps) -> Element {
    let SystemSlotKeyPresentation {
        label,
        compact,
        conflict,
    } = SystemSlotKeyPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            "data-compact": compact,
            "data-conflict": conflict,
            {label}
        }
    }
}
