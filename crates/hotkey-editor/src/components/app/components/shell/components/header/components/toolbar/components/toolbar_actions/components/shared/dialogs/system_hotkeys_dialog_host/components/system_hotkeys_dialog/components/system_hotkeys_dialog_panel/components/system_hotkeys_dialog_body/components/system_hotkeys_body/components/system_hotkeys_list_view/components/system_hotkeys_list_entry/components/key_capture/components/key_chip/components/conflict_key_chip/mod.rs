mod props;
mod view;

pub use view::ConflictKeyChipView;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
use props::ConflictKeyChipProps;
use style::CLASS;
use tw_macro::assert_component;

/// The red, conflicting look of a system-hotkey chip: props in, markup out. It bakes
/// its own danger colour into `style.rs` and owns the classed button root.
#[component]
pub fn ConflictKeyChip(props: ConflictKeyChipProps) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let text = props.tooltip_text;
    let placement = props.tooltip_placement;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
            Tooltip { text, placement }
        }
    }
}

assert_component!(ConflictKeyChip);
