use super::components::conflict_slot::ConflictSlotProps;
use super::components::highlighted_slot::HighlightedSlotProps;
use super::components::idle_slot::IdleSlotProps;
use super::components::shared::slot_contents::SlotContentsProps;
use super::props::SystemSlotProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::shared::tooltip::{TooltipAnchor, TooltipProps};

impl From<&SystemSlotProps> for SystemSlotLabelProps {
    fn from(props: &SystemSlotProps) -> Self {
        let text = props.slot_label.clone();
        Self { text }
    }
}

impl From<&SystemSlotProps> for SystemSlotKeyProps {
    fn from(props: &SystemSlotProps) -> Self {
        let label = props.key_label.clone();
        let conflict = props.conflict;
        Self { label, conflict }
    }
}

impl From<&SystemSlotProps> for TooltipProps {
    fn from(props: &SystemSlotProps) -> Self {
        let text = props.tooltip_text.clone();
        let placement = props.tooltip_placement;
        let anchor = TooltipAnchor::Center;
        Self {
            text,
            placement,
            anchor,
        }
    }
}

impl From<&SystemSlotProps> for SlotContentsProps {
    fn from(props: &SystemSlotProps) -> Self {
        let label = SystemSlotLabelProps::from(props);
        let slot_key = SystemSlotKeyProps::from(props);
        let tooltip = TooltipProps::from(props);
        let dragging = props.dragging;
        Self {
            label,
            slot_key,
            tooltip,
            dragging,
        }
    }
}

impl From<&SystemSlotProps> for IdleSlotProps {
    fn from(props: &SystemSlotProps) -> Self {
        let contents = SlotContentsProps::from(props);
        Self { contents }
    }
}

impl From<&SystemSlotProps> for HighlightedSlotProps {
    fn from(props: &SystemSlotProps) -> Self {
        let contents = SlotContentsProps::from(props);
        Self { contents }
    }
}

impl From<&SystemSlotProps> for ConflictSlotProps {
    fn from(props: &SystemSlotProps) -> Self {
        let contents = SlotContentsProps::from(props);
        Self { contents }
    }
}
