/// The published `View` contract mirroring [`EditingKeycapProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EditingKeycapView {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    pub label: String,
}

impl ddd::View for EditingKeycapView {}
