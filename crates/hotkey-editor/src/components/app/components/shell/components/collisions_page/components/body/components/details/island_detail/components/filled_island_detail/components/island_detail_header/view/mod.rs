use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`IslandDetailHeaderModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IslandDetailHeaderView {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl ddd::View for IslandDetailHeaderView {}
