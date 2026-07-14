use super::view::UnitPortraitHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitPortraitHostModel {
    pub src: Option<String>,
    pub alt: &'static str,
}

impl From<&UnitPortraitHostView> for UnitPortraitHostModel {
    fn from(view: &UnitPortraitHostView) -> Self {
        let UnitPortraitHostView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for UnitPortraitHostModel {
    type View = UnitPortraitHostView;
}
