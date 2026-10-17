use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct TopConflictPositionView {
    pub coordinate: GridCoordinate,
}

impl ddd::Layered for TopConflictPositionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TopConflictPositionView {}
