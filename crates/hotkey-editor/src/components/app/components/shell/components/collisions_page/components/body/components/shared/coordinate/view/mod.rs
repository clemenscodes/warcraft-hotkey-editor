use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct CoordinateView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for CoordinateView {}
