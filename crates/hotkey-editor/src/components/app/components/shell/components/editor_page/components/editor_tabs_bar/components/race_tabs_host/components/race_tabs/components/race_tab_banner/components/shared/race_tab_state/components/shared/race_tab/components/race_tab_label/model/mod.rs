use super::view::RaceTabLabelView;
use dioxus::prelude::*;

/// The race label: the display name to show. Its colour is inherited from the tab
/// button's `--label-color`, so the label needs no race or state of its own.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabLabelModel {
    pub label: String,
}

impl From<&RaceTabLabelView> for RaceTabLabelModel {
    fn from(view: &RaceTabLabelView) -> Self {
        let RaceTabLabelView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Model for RaceTabLabelModel {
    type View = RaceTabLabelView;
}
