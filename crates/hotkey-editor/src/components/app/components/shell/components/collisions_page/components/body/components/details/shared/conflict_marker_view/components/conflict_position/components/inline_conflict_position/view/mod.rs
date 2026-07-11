use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`InlineConflictPositionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct InlineConflictPositionView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for InlineConflictPositionView {}
