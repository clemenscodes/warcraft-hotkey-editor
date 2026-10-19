use super::view::UnitDetailButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnitDetailButtonModel {
    pub src: Option<String>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&UnitDetailButtonView> for UnitDetailButtonModel {
    fn from(view: &UnitDetailButtonView) -> Self {
        let UnitDetailButtonView { src, onclick } = view.clone();
        Self { src, onclick }
    }
}

impl ddd::Layered for UnitDetailButtonModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnitDetailButtonModel {
    type View = UnitDetailButtonView;
}
