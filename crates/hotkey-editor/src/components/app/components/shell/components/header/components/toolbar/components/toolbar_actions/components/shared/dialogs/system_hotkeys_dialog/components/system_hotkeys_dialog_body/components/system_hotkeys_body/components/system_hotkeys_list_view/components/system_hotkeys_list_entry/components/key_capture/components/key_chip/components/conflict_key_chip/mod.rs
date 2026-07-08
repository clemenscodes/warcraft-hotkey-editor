mod props;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::ConflictKeyChipProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictKeyChip);

/// The red, conflicting look of a system-hotkey chip: props in, markup out. It bakes
/// its own danger colour into `style.rs` and owns the classed button root.
#[component]
pub fn ConflictKeyChip(props: ConflictKeyChipProps) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let tooltip = props.tooltip;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
            Tooltip { ..tooltip }
        }
    }
}
