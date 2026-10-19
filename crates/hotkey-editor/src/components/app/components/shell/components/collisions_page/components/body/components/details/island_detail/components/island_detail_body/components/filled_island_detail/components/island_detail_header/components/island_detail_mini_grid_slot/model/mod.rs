use super::view::IslandDetailMiniGridSlotView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct IslandDetailMiniGridSlotModel {
    pub coordinate: GridCoordinate,
}

impl From<&IslandDetailMiniGridSlotView> for IslandDetailMiniGridSlotModel {
    fn from(view: &IslandDetailMiniGridSlotView) -> Self {
        let IslandDetailMiniGridSlotView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Layered for IslandDetailMiniGridSlotModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for IslandDetailMiniGridSlotModel {
    type View = IslandDetailMiniGridSlotView;
}
