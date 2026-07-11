use super::view::TileIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileIconModel {
    /// The ability icon, or `None` when the occupant has no icon (then the
    /// sibling `TileLabel` renders the text fallback instead).
    pub src: Option<String>,
    pub alt: String,
}

impl From<&TileIconView> for TileIconModel {
    fn from(view: &TileIconView) -> Self {
        let TileIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for TileIconModel {
    type View = TileIconView;
}
