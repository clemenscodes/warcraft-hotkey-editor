use super::view::PagerCardPortraitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardPortraitModel {
    pub src: Option<String>,
}

impl From<&PagerCardPortraitView> for PagerCardPortraitModel {
    fn from(view: &PagerCardPortraitView) -> Self {
        let PagerCardPortraitView { src } = view.clone();
        Self { src }
    }
}

impl ddd::Model for PagerCardPortraitModel {
    type View = PagerCardPortraitView;
}
