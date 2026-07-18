use super::view::RaceTabLabelView;
use dioxus::prelude::*;

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
