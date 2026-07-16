use super::view::PagerSpacerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PagerSpacerModel {
    pub height_px: i32,
}

impl From<&PagerSpacerView> for PagerSpacerModel {
    fn from(view: &PagerSpacerView) -> Self {
        let PagerSpacerView { height_px } = view.clone();
        Self { height_px }
    }
}

impl ddd::Model for PagerSpacerModel {
    type View = PagerSpacerView;
}
