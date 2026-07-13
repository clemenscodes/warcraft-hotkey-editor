mod model;
mod view;

pub use view::SlotContentsView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKey;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabel;
use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
use model::SlotContentsModel;
use tw_macro::assert_component;

/// The framed slot's inner content — caption, bound key, and conflict tooltip —
/// shared by every slot look. While the slot is being dragged the content is
/// unmounted rather than hidden, so the frame's DOM node stays mounted and the
/// pointer-capture drag on the host is never interrupted by a class swap.
#[component]
pub fn SlotContents(props: SlotContentsModel) -> Element {
    if props.dragging {
        return rsx! {};
    }
    let slot_label = props.slot_label;
    let key_label = props.key_label;
    let conflict = props.conflict;
    let tooltip_text = props.tooltip_text;
    let tooltip_placement = props.tooltip_placement;
    rsx! {
        SystemSlotLabel { text: slot_label }
        SystemSlotKey {
            label: key_label,
            conflict,
        }
        Tooltip {
            text: tooltip_text,
            placement: tooltip_placement,
        }
    }
}

assert_component!(SlotContents);
