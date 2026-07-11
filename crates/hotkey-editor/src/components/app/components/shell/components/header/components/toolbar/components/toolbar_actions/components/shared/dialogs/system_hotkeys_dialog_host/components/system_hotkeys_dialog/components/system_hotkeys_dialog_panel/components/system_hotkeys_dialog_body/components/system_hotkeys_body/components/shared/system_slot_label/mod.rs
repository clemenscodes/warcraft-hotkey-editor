mod model;
mod view;

pub use view::SystemSlotLabelView;
mod style;

use dioxus::prelude::*;
use model::SystemSlotLabelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The caption shown above the key on a system hotkey slot.
#[component]
pub fn SystemSlotLabel(props: SystemSlotLabelModel) -> Element {
    let text = props.text;
    rsx! {
        div {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(SystemSlotLabel);
