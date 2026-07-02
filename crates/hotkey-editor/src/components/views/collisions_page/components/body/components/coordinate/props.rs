use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// A command-card coordinate. Carries the domain `GridCoordinate`; the leaf just
/// displays its column and row.
#[derive(Props, Clone, PartialEq)]
pub struct CoordinateProps {
    pub coordinate: GridCoordinate,
}
