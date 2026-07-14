use super::view::InlineConflictPositionView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

#[derive(Props, Clone, PartialEq)]
pub struct InlineConflictPositionModel {
    pub coordinate: GridCoordinate,
}

impl From<&InlineConflictPositionView> for InlineConflictPositionModel {
    fn from(view: &InlineConflictPositionView) -> Self {
        let InlineConflictPositionView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Model for InlineConflictPositionModel {
    type View = InlineConflictPositionView;
}
