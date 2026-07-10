mod logic;
mod props;
mod style;

use dioxus::prelude::*;
use logic::SystemSlotLabelPresentation;
pub use props::SystemSlotLabelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemSlotLabel);

/// The caption shown above the key on a system hotkey slot.
#[component]
pub fn SystemSlotLabel(props: SystemSlotLabelProps) -> Element {
    let SystemSlotLabelPresentation { text, compact } = SystemSlotLabelPresentation::from(&props);
    rsx! {
        div { class: CLASS, "data-compact": compact, {text} }
    }
}
