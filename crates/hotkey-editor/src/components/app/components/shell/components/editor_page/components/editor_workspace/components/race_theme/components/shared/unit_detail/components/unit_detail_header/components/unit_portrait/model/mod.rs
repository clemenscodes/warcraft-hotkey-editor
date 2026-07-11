use super::view::UnitPortraitView;
use dioxus::prelude::*;

/// The unit portrait: its source (absent for units without an image) and alt text.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPortraitModel {
    pub src: Option<String>,
    pub alt: &'static str,
}

impl From<&UnitPortraitView> for UnitPortraitModel {
    fn from(view: &UnitPortraitView) -> Self {
        let UnitPortraitView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for UnitPortraitModel {
    type View = UnitPortraitView;
}
