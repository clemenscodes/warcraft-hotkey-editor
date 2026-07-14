use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct InlineConflictPositionView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for InlineConflictPositionView {}
