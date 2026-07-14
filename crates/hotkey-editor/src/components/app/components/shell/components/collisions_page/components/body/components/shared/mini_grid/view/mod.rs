use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for MiniGridView {}
