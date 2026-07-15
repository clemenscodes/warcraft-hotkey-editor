use super::view::HiddenCategoryChipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HiddenCategoryChipModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&HiddenCategoryChipView> for HiddenCategoryChipModel {
    fn from(view: &HiddenCategoryChipView) -> Self {
        let HiddenCategoryChipView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for HiddenCategoryChipModel {
    type View = HiddenCategoryChipView;
}
