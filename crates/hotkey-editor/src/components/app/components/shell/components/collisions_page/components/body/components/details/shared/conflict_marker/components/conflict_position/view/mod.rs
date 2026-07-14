use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct ConflictPositionView {
    pub coordinate: GridCoordinate,
    pub is_top: bool,
}

impl ddd::View for ConflictPositionView {}
