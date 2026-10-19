use warcraft_keybinds::GridCoordinate;

#[derive(Clone, PartialEq)]
pub struct IslandRowMiniGridSlotView {
    pub coordinate: GridCoordinate,
}

impl ddd::Layered for IslandRowMiniGridSlotView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IslandRowMiniGridSlotView {}
