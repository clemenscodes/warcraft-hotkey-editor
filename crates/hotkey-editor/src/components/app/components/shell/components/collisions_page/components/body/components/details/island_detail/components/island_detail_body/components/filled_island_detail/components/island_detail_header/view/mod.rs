use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct IslandDetailHeaderView {
    pub coordinate: GridCoordinate,
    pub count: usize,
}

impl ddd::Layered for IslandDetailHeaderView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandDetailHeaderView {}
