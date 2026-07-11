use super::view::CoordinateView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// A command-card coordinate. Carries the domain `GridCoordinate`; the leaf just
/// displays its column and row.
#[derive(Props, Clone, PartialEq)]
pub struct CoordinateModel {
    pub coordinate: GridCoordinate,
}

impl From<&CoordinateView> for CoordinateModel {
    fn from(view: &CoordinateView) -> Self {
        let CoordinateView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Model for CoordinateModel {
    type View = CoordinateView;
}
