use super::props::SystemSlotLabelProps;

/// The caption's shaped presentation: the text plus the compact flag as the
/// `"true"` / `"false"` attribute string its `data-*` variant matches on.
pub(super) struct SystemSlotLabelPresentation {
    pub(super) text: String,
    pub(super) compact: &'static str,
}

impl From<&SystemSlotLabelProps> for SystemSlotLabelPresentation {
    fn from(props: &SystemSlotLabelProps) -> Self {
        let text = props.text.clone();
        let compact = if props.compact { "true" } else { "false" };
        Self { text, compact }
    }
}
