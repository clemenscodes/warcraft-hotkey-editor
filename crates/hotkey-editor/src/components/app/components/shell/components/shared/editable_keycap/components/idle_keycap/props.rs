use crate::components::app::components::shell::components::shared::editable_keycap::{
    EditableKeycapProps, EditableKeycapRadius,
};
use dioxus::prelude::*;

/// The resting editable keycap: the glyph it shows and its corner radius. Built by the
/// `EditableKeycap` dispatcher from its props when the cap is `Idle`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleKeycapProps {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
    /// The corner radius variant the host asks for.
    pub radius: EditableKeycapRadius,
}

impl From<&EditableKeycapProps> for IdleKeycapProps {
    fn from(props: &EditableKeycapProps) -> Self {
        let label = props.label.clone();
        let radius = props.radius;
        Self { label, radius }
    }
}
