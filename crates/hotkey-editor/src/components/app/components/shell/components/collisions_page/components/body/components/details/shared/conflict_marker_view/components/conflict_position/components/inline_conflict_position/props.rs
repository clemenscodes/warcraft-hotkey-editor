use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The colliding cell shown inline between a conflict's two abilities.
#[derive(Props, Clone, PartialEq)]
pub struct InlineConflictPositionProps {
    pub coordinate: GridCoordinate,
}
