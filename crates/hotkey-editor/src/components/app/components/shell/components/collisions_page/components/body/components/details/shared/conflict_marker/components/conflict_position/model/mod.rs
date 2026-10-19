use super::view::ConflictPositionView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictPositionModel {
    pub coordinate: GridCoordinate,
    #[props(default)]
    pub is_top: bool,
}

impl From<&ConflictPositionView> for ConflictPositionModel {
    fn from(view: &ConflictPositionView) -> Self {
        let ConflictPositionView { coordinate, is_top } = view.clone();
        Self { coordinate, is_top }
    }
}

impl ddd::Layered for ConflictPositionModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ConflictPositionModel {
    type View = ConflictPositionView;
}
