use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`TopConflictPositionProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TopConflictPositionView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for TopConflictPositionView {}
