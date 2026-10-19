use super::view::IslandRowMiniGridSlotView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct IslandRowMiniGridSlotModel {
    pub coordinate: GridCoordinate,
}

impl From<&IslandRowMiniGridSlotView> for IslandRowMiniGridSlotModel {
    fn from(view: &IslandRowMiniGridSlotView) -> Self {
        let IslandRowMiniGridSlotView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Layered for IslandRowMiniGridSlotModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for IslandRowMiniGridSlotModel {
    type View = IslandRowMiniGridSlotView;
}
