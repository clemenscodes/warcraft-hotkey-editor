use super::view::UnitPortraitView;
use dioxus::prelude::*;

/// The unit portrait: its source (absent for units without an image) and alt text.
#[derive(Props, Clone, PartialEq)]
pub struct UnitPortraitProps {
    pub src: Option<String>,
    pub alt: &'static str,
}

impl From<&UnitPortraitView> for UnitPortraitProps {
    fn from(view: &UnitPortraitView) -> Self {
        let UnitPortraitView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for UnitPortraitProps {
    type View = UnitPortraitView;
}
