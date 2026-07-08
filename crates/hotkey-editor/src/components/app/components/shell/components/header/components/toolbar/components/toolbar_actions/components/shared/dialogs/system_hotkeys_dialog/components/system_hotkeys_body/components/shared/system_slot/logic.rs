use super::props::SystemSlotProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::shared::system_slot_key::SystemSlotKeyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_body::components::shared::system_slot_label::SystemSlotLabelProps;
use crate::components::app::components::shell::components::shared::tooltip::{TooltipAnchor, TooltipProps};

impl From<&SystemSlotProps> for SystemSlotLabelProps {
    fn from(props: &SystemSlotProps) -> Self {
        let text = props.slot_label.clone();
        let compact = props.compact;
        Self { text, compact }
    }
}

impl From<&SystemSlotProps> for SystemSlotKeyProps {
    fn from(props: &SystemSlotProps) -> Self {
        let label = props.key_label.clone();
        let compact = props.compact;
        let conflict = props.conflict;
        Self {
            label,
            compact,
            conflict,
        }
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
