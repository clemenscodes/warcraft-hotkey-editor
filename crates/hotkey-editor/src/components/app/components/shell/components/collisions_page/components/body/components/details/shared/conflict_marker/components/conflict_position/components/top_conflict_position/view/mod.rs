use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct TopConflictPositionView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for TopConflictPositionView {}
