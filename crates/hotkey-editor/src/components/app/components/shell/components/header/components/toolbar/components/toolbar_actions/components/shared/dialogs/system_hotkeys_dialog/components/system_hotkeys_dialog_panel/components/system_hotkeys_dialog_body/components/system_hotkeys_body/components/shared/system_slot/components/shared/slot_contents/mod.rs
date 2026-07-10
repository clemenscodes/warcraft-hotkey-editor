mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKey;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabel;
use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
pub use props::SlotContentsProps;
use tw_macro::assert_component;
assert_component!(SlotContents);

/// The framed slot's inner content — caption, bound key, and conflict tooltip —
/// shared by every slot look. While the slot is being dragged the content is
/// unmounted rather than hidden, so the frame's DOM node stays mounted and the
/// pointer-capture drag on the host is never interrupted by a class swap.
#[component]
pub fn SlotContents(props: SlotContentsProps) -> Element {
    if props.dragging {
        return rsx! {};
    }
    let label = props.label;
    let slot_key = props.slot_key;
    let tooltip = props.tooltip;
    rsx! {
        SystemSlotLabel { ..label }
        SystemSlotKey { ..slot_key }
        Tooltip { ..tooltip }
    }
}
