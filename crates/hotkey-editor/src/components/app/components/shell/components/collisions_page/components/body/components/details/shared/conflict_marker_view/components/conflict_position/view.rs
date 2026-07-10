use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`ConflictPositionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictPositionView {
    pub coordinate: GridCoordinate,
    pub is_top: bool,
}

impl ddd::View for ConflictPositionView {}
