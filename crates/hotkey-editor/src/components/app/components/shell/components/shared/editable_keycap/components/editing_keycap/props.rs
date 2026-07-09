use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapProps;
use dioxus::prelude::*;

/// The pulsing editable keycap: the glyph it shows. Built by the `EditableKeycap`
/// dispatcher from its props when the cap is `Editing`. Its corner radius comes from the
/// inherited `--keycap-radius`.
#[derive(Props, Clone, PartialEq)]
pub struct EditingKeycapProps {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
}

impl From<&EditableKeycapProps> for EditingKeycapProps {
    fn from(props: &EditableKeycapProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
