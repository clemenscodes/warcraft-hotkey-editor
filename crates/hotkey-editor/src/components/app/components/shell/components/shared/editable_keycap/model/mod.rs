use super::state::EditableKeycapState;
use super::view::EditableKeycapView;
use dioxus::prelude::*;

/// The shared editable gold keycap: the glyph it shows and its capture pulse. Purely
/// presentational — its host button owns size, focus, and every event handler; this leaf
/// owns the cap look. Its corner radius comes from the inherited `--keycap-radius` (panel
/// when unset), which the host sets. The gallery can render it directly.
#[derive(Props, Clone, PartialEq)]
pub struct EditableKeycapModel {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
    /// Whether the cap is resting or pulsing gold while its key picker is open.
    #[props(default)]
    pub state: EditableKeycapState,
}

impl From<&EditableKeycapView> for EditableKeycapModel {
    fn from(view: &EditableKeycapView) -> Self {
        let EditableKeycapView { label, state } = view.clone();
        Self { label, state }
    }
}

impl ddd::Model for EditableKeycapModel {
    type View = EditableKeycapView;
}
