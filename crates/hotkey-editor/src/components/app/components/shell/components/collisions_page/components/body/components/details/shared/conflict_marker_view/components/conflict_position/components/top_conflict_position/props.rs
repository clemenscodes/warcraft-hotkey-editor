use super::view::TopConflictPositionView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The colliding cell stacked over a multi-way ability row.
#[derive(Props, Clone, PartialEq)]
pub struct TopConflictPositionProps {
    pub coordinate: GridCoordinate,
}

impl From<&TopConflictPositionView> for TopConflictPositionProps {
    fn from(view: &TopConflictPositionView) -> Self {
        let TopConflictPositionView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Props for TopConflictPositionProps {
    type View = TopConflictPositionView;
}
