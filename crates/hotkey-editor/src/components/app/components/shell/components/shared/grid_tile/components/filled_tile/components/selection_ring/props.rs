use super::super::super::props::FilledTileProps;
use dioxus::prelude::*;

/// The selection ring mounts only for the currently selected tile; every other tile
/// early-returns, so its mere presence is the tile's selected signal.
#[derive(Props, Clone, PartialEq)]
pub struct SelectionRingProps {
    pub selected: bool,
}

impl From<&FilledTileProps> for SelectionRingProps {
    fn from(props: &FilledTileProps) -> Self {
        let selected = props.selected;
        Self { selected }
    }
}
