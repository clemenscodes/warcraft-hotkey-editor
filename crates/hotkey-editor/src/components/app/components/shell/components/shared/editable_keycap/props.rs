use super::state::{EditableKeycapRadius, EditableKeycapState};
use dioxus::prelude::*;

/// The shared editable gold keycap: the glyph it shows, its corner radius, and its
/// capture pulse. Purely presentational — its host button owns size, focus, and every
/// event handler; this leaf owns the cap look. The gallery can render it directly.
#[derive(Props, Clone, PartialEq)]
pub struct EditableKeycapProps {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
    /// The corner radius variant the host asks for.
    pub radius: EditableKeycapRadius,
    /// Whether the cap is resting or pulsing gold while its key picker is open.
    #[props(default)]
    pub state: EditableKeycapState,
}
