use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileIconProps {
    /// The ability icon, or `None` when the occupant has no icon (then the
    /// sibling `TileLabel` renders the text fallback instead).
    pub src: Option<String>,
    pub alt: String,
}
