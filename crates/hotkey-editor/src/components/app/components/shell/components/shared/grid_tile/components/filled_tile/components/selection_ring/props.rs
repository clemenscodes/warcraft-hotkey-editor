use super::view::SelectionRingView;
use dioxus::prelude::*;

/// The selection ring mounts only for the currently selected tile; every other tile
/// leaves `selected` false and early-returns, so its mere presence is the tile's
/// selected signal.
#[derive(Props, Clone, PartialEq)]
pub struct SelectionRingProps {
    pub selected: bool,
}

impl From<&SelectionRingView> for SelectionRingProps {
    fn from(view: &SelectionRingView) -> Self {
        let SelectionRingView { selected } = view.clone();
        Self { selected }
    }
}

impl ddd::Props for SelectionRingProps {
    type View = SelectionRingView;
}
