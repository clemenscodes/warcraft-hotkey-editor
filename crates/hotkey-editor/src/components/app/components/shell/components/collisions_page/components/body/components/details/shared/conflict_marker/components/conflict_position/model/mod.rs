use super::view::ConflictPositionView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The colliding command-card cell shown between (or above) a conflict's abilities;
/// `is_top` stacks it over a multi-way ability row.
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

impl ddd::Model for ConflictPositionModel {
    type View = ConflictPositionView;
}
