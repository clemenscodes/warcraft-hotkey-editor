use super::view::ShownCategoryChipView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ShownCategoryChipModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ShownCategoryChipView> for ShownCategoryChipModel {
    fn from(view: &ShownCategoryChipView) -> Self {
        let ShownCategoryChipView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for ShownCategoryChipModel {
    type View = ShownCategoryChipView;
}
