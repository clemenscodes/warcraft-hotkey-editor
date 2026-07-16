use super::view::PagerCardNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardNameModel {
    #[props(into)]
    pub name: String,
}

impl From<&PagerCardNameView> for PagerCardNameModel {
    fn from(view: &PagerCardNameView) -> Self {
        let PagerCardNameView { name } = view.clone();
        Self { name }
    }
}

impl ddd::Model for PagerCardNameModel {
    type View = PagerCardNameView;
}
