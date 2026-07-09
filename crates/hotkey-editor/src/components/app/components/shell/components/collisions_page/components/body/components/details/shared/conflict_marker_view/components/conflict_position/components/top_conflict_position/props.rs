use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The colliding cell stacked over a multi-way ability row.
#[derive(Props, Clone, PartialEq)]
pub struct TopConflictPositionProps {
    pub coordinate: GridCoordinate,
}
