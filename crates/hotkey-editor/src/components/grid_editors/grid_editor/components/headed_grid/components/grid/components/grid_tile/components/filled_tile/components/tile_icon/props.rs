use dioxus::prelude::*;

use super::super::super::props::FilledTileProps;

#[derive(Props, Clone, PartialEq)]
pub struct TileIconProps {
    /// The ability icon, or `None` when the occupant has no icon (then the
    /// sibling `TileLabel` renders the text fallback instead).
    pub src: Option<String>,
    pub alt: String,
}

impl From<&FilledTileProps> for TileIconProps {
    fn from(props: &FilledTileProps) -> Self {
        let src = props.icon.clone();
        let alt = props.label.clone();
        Self { src, alt }
    }
}
