use super::view::RaceScopePanelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceScopePanelModel {
    pub on_back: EventHandler<MouseEvent>,
}

impl From<&RaceScopePanelView> for RaceScopePanelModel {
    fn from(view: &RaceScopePanelView) -> Self {
        let RaceScopePanelView { on_back } = view.clone();
        Self { on_back }
    }
}

impl ddd::Model for RaceScopePanelModel {
    type View = RaceScopePanelView;
}
