/// The published `View` contract mirroring [`TileIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileIconView {
    /// The ability icon, or `None` when the occupant has no icon (then the
    /// sibling `TileLabel` renders the text fallback instead).
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for TileIconView {}
