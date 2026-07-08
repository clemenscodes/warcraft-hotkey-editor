use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::shared::tooltip::TooltipProps;
use dioxus::prelude::*;

/// The conflict-look slot's props: the already-shaped caption, bound-key, and tooltip
/// child props, plus the two orthogonal flags — `compact` (the tighter control-group
/// cell) and `dragging` (this slot is being dragged, so its contents hide). Built by
/// the dispatcher from `SystemSlotProps`; carrying the child props as data is passing
/// data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictSlotProps {
    pub label: SystemSlotLabelProps,
    pub key: SystemSlotKeyProps,
    pub tooltip: TooltipProps,
    pub compact: bool,
    pub dragging: bool,
}
