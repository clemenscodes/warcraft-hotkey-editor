use super::state::EditableKeycapState;

/// The published `View` contract mirroring [`EditableKeycapProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EditableKeycapView {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    pub label: String,
    /// Whether the cap is resting or pulsing gold while its key picker is open.
    pub state: EditableKeycapState,
}

impl ddd::View for EditableKeycapView {}
