use dioxus::prelude::*;

/// One cell of the island mini grid; the collision cell is highlighted.
#[derive(Props, Clone, PartialEq)]
pub struct IslandMiniCellProps {
    pub is_collision: bool,
}
