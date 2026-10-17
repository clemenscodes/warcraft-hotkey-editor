use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct CoordinateView {
    pub coordinate: GridCoordinate,
}

impl ddd::Layered for CoordinateView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for CoordinateView {}
