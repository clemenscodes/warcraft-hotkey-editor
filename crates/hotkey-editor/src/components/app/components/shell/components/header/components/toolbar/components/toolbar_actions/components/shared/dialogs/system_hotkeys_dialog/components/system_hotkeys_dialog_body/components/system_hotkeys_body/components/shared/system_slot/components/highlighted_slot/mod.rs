mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKey;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabel;
use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::HighlightedSlotProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HighlightedSlot);

/// The highlighted look of a system slot: the gold border-image frame with a gold
/// glow overlay, composing the caption, bound key, and conflict tooltip.
/// Presentational — the dispatcher builds its props and renders it when the slot's
/// glow state is highlighted.
#[component]
pub fn HighlightedSlot(props: HighlightedSlotProps) -> Element {
    let label = props.label;
    let slot_key = props.slot_key;
    let tooltip = props.tooltip;
    rsx! {
        div {
            class: CLASS,
            "data-compact": props.compact,
            "data-dragging": props.dragging,
            SystemSlotLabel { ..label }
            SystemSlotKey { ..slot_key }
            Tooltip { ..tooltip }
        }
    }
}
