use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The colliding command-card cell shown between (or above) a conflict's abilities;
/// `is_top` stacks it over a multi-way ability row.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictPositionCellProps {
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub is_top: bool,
}
