use super::view::InlineConflictPositionView;
use dioxus::prelude::*;
use warcraft_keybinds::GridCoordinate;

/// The colliding cell shown inline between a conflict's two abilities.
#[derive(Props, Clone, PartialEq)]
pub struct InlineConflictPositionProps {
    pub coordinate: GridCoordinate,
}

impl From<&InlineConflictPositionView> for InlineConflictPositionProps {
    fn from(view: &InlineConflictPositionView) -> Self {
        let InlineConflictPositionView { coordinate } = view.clone();
        Self { coordinate }
    }
}

impl ddd::Props for InlineConflictPositionProps {
    type View = InlineConflictPositionView;
}
