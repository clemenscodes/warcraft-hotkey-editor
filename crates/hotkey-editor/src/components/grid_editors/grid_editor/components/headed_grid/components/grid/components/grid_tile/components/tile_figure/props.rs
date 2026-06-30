use dioxus::prelude::*;

use super::super::super::GridTileProps;

#[derive(Props, Clone, PartialEq)]
pub struct TileFigureProps {
    /// The ability icon, or `None` to fall back to the text label.
    pub icon: Option<String>,
    pub alt: String,
    /// Whether the tile is focusable; the text label only shows when it is.
    pub is_focusable: bool,
}

impl From<&GridTileProps> for TileFigureProps {
    fn from(props: &GridTileProps) -> Self {
        let icon = props.icon.clone();
        let alt = props.label.clone();
        let is_focusable = props.is_focusable;
        Self {
            icon,
            alt,
            is_focusable,
        }
    }
}
