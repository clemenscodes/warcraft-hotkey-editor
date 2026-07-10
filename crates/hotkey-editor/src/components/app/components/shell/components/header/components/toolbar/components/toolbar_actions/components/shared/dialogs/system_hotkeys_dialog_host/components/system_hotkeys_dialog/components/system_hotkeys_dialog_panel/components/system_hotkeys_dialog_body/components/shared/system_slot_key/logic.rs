use super::components::conflict_slot_key::ConflictSlotKeyProps;
use super::components::plain_slot_key::PlainSlotKeyProps;
use super::props::SystemSlotKeyProps;

impl From<&SystemSlotKeyProps> for PlainSlotKeyProps {
    fn from(props: &SystemSlotKeyProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}

impl From<&SystemSlotKeyProps> for ConflictSlotKeyProps {
    fn from(props: &SystemSlotKeyProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
