use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct ConflictPositionView {
    pub coordinate: GridCoordinate,
    pub is_top: bool,
}

impl ddd::Layered for ConflictPositionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictPositionView {}
