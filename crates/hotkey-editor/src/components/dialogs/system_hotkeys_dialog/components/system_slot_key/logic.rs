use super::props::SystemSlotKeyProps;

/// The key's shaped presentation: the label plus the two flags as the `"true"` /
/// `"false"` attribute strings the `data-*` style variants match on.
pub(super) struct SystemSlotKeyPresentation {
    pub(super) label: String,
    pub(super) compact: &'static str,
    pub(super) conflict: &'static str,
}

impl From<&SystemSlotKeyProps> for SystemSlotKeyPresentation {
    fn from(props: &SystemSlotKeyProps) -> Self {
        let label = props.label.clone();
        let compact = if props.compact { "true" } else { "false" };
        let conflict = if props.conflict { "true" } else { "false" };
        Self {
            label,
            compact,
            conflict,
        }
    }
}
