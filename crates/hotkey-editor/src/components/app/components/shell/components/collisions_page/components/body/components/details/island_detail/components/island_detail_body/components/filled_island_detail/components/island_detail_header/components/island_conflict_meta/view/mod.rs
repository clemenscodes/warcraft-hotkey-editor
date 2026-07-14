use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct IslandConflictMetaView {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl ddd::View for IslandConflictMetaView {}
