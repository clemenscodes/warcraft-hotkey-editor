use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct IslandDetailHeaderView {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl ddd::View for IslandDetailHeaderView {}
