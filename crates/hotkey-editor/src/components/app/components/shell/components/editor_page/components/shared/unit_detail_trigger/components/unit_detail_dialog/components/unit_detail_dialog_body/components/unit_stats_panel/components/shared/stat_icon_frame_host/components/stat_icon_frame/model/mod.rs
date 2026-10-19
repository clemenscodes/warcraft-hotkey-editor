use super::view::StatIconFrameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StatIconFrameModel {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}

impl From<&StatIconFrameView> for StatIconFrameModel {
    fn from(view: &StatIconFrameView) -> Self {
        let StatIconFrameView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Layered for StatIconFrameModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for StatIconFrameModel {
    type View = StatIconFrameView;
}
