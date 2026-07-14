use super::view::SelectionRingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SelectionRingModel {
    pub selected: bool,
}

impl From<&SelectionRingView> for SelectionRingModel {
    fn from(view: &SelectionRingView) -> Self {
        let SelectionRingView { selected } = view.clone();
        Self { selected }
    }
}

impl ddd::Model for SelectionRingModel {
    type View = SelectionRingView;
}
