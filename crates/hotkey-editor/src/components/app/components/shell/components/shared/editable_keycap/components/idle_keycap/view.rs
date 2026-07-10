/// The published `View` contract mirroring [`IdleKeycapProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IdleKeycapView {
    /// The visible glyph — a single letter, "–", "Esc", "Mouse4", etc.
    pub label: String,
}

impl ddd::View for IdleKeycapView {}
