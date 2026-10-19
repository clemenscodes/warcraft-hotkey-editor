use super::view::RaceTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RaceTabModel {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceTabView> for RaceTabModel {
    fn from(view: &RaceTabView) -> Self {
        let RaceTabView {
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Layered for RaceTabModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RaceTabModel {
    type View = RaceTabView;
}
