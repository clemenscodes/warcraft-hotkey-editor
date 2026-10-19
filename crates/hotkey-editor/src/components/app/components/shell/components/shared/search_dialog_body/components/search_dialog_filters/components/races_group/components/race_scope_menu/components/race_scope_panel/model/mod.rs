use super::view::RaceScopePanelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopePanelModel {
    pub leaving: bool,
}

impl From<&RaceScopePanelView> for RaceScopePanelModel {
    fn from(view: &RaceScopePanelView) -> Self {
        let RaceScopePanelView { leaving } = *view;
        Self { leaving }
    }
}

impl ddd::Layered for RaceScopePanelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RaceScopePanelModel {
    type View = RaceScopePanelView;
}
