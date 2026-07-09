use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapProps;
use dioxus::prelude::*;

/// The resting editable keycap: the glyph it shows. Built by the `EditableKeycap`
/// dispatcher from its props when the cap is `Idle`. Its corner radius comes from the
/// inherited `--keycap-radius`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleKeycapProps {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
}

impl From<&EditableKeycapProps> for IdleKeycapProps {
    fn from(props: &EditableKeycapProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}
