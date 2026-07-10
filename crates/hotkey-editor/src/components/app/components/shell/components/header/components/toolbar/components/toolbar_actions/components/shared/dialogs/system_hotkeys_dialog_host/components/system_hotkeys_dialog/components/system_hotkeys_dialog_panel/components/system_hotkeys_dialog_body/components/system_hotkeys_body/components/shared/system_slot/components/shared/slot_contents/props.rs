use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// The framed slot's inner content: the already-shaped caption, bound-key, and
/// tooltip child props, plus the `dragging` flag. While the slot is being dragged the
/// content is unmounted (not merely hidden), so the frame's root DOM node stays
/// mounted and the pointer-capture drag on the host is never interrupted by a class
/// swap on a live element.
#[derive(Props, Clone, PartialEq)]
pub struct SlotContentsProps {
    pub label: SystemSlotLabelProps,
    pub slot_key: SystemSlotKeyProps,
    pub tooltip: TooltipProps,
    pub dragging: bool,
}
