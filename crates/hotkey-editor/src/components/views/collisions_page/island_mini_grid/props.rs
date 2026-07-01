use dioxus::prelude::*;

/// A tiny 4×3 command grid highlighting only the island's conflicting cell.
#[derive(Props, Clone, PartialEq)]
pub struct IslandMiniGridProps {
    pub collision_column: u8,
    pub collision_row: u8,
}
