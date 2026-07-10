use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`MiniGridProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for MiniGridView {}
