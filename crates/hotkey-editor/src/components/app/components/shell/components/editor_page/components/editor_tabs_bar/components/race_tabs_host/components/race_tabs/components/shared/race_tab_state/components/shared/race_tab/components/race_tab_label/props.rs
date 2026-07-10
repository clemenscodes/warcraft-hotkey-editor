use super::view::RaceTabLabelView;
use dioxus::prelude::*;

/// The race label: the display name to show. Its colour is inherited from the tab
/// button's `--label-color`, so the label needs no race or state of its own.
#[derive(Props, Clone, PartialEq)]
pub struct RaceTabLabelProps {
    pub label: String,
}

impl From<&RaceTabLabelView> for RaceTabLabelProps {
    fn from(view: &RaceTabLabelView) -> Self {
        let RaceTabLabelView { label } = view.clone();
        Self { label }
    }
}

impl ddd::Props for RaceTabLabelProps {
    type View = RaceTabLabelView;
}
