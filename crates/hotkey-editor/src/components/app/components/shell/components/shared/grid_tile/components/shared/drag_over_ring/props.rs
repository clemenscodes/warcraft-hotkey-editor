use super::super::super::empty_tile::EmptyTileProps;
use super::super::super::filled_tile::FilledTileProps;
use dioxus::prelude::*;

/// Mounts only on the tile the cursor hovers during a drag; every other tile
/// early-returns, so its presence is the under-cursor signal the tile root's gold border
/// keys off. Shared by the filled and empty tiles, so it is built from either.
#[derive(Props, Clone, PartialEq)]
pub struct DragOverRingProps {
    pub active: bool,
}

impl From<&FilledTileProps> for DragOverRingProps {
    fn from(props: &FilledTileProps) -> Self {
        let active = props.is_drag_over;
        Self { active }
    }
}

impl From<&EmptyTileProps> for DragOverRingProps {
    fn from(props: &EmptyTileProps) -> Self {
        let active = props.is_drag_over;
        Self { active }
    }
}
