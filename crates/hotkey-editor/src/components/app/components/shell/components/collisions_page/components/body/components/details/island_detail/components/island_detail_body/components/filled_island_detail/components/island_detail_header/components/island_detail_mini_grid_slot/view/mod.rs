use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct IslandDetailMiniGridSlotView {
    pub coordinate: GridCoordinate,
}

impl ddd::Layered for IslandDetailMiniGridSlotView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandDetailMiniGridSlotView {}
