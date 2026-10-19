use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct IslandConflictMetaView {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl ddd::Layered for IslandConflictMetaView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandConflictMetaView {}
