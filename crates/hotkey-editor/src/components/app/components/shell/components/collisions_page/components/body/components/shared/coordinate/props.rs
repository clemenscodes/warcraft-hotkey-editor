use super::view::CoordinateView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// A command-card coordinate. Carries the domain `GridCoordinate`; the leaf just
/// displays its column and row.
#[derive(Props, Clone, PartialEq)]
pub struct CoordinateProps {
    pub coordinate: GridCoordinate,
}

impl From<&CoordinateView> for CoordinateProps {
    fn from(view: &CoordinateView) -> Self {
        let CoordinateView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Props for CoordinateProps {
    type View = CoordinateView;
}
