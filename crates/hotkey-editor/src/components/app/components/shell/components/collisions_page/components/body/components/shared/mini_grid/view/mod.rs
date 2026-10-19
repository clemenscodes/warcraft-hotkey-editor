use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct MiniGridView {
    pub coordinate: GridCoordinate,
}

impl ddd::Layered for MiniGridView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for MiniGridView {}
