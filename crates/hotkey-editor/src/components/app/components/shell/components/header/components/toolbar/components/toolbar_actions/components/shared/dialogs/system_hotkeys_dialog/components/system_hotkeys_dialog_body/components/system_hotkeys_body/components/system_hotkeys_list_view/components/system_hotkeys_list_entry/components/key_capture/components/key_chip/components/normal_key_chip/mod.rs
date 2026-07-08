mod props;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::NormalKeyChipProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NormalKeyChip);

/// The gold, non-conflicting look of a system-hotkey chip: props in, markup out. It
/// bakes its own gold colour into `style.rs` and owns the classed button root.
#[component]
pub fn NormalKeyChip(props: NormalKeyChipProps) -> Element {
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
