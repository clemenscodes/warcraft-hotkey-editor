use super::view::UnitDetailPortraitView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailPortraitModel {
    pub src: Option<String>,
}

impl From<&UnitDetailPortraitView> for UnitDetailPortraitModel {
    fn from(view: &UnitDetailPortraitView) -> Self {
        let UnitDetailPortraitView { src } = view.clone();
        Self { src }
    }
}

impl ddd::Layered for UnitDetailPortraitModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitDetailPortraitModel {
    type View = UnitDetailPortraitView;
}
