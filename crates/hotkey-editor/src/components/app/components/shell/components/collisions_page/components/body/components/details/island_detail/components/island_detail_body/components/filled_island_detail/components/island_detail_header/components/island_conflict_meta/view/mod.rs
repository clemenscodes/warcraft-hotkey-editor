use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`IslandConflictMetaModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandConflictMetaView {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl ddd::View for IslandConflictMetaView {}
