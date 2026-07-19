use super::view::PagerCardDetailButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PagerCardDetailButtonModel {
    pub src: Option<String>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&PagerCardDetailButtonView> for PagerCardDetailButtonModel {
    fn from(view: &PagerCardDetailButtonView) -> Self {
        let PagerCardDetailButtonView { src, onclick } = view.clone();
        Self { src, onclick }
    }
}

impl ddd::Model for PagerCardDetailButtonModel {
    type View = PagerCardDetailButtonView;
}
