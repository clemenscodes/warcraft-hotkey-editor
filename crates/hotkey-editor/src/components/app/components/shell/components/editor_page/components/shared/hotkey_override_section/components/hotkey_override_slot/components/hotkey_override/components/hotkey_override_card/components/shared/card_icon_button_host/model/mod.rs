use super::view::CardIconButtonHostView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardIconButtonHostModel {
    #[props(into)]
    pub title: String,
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl From<&CardIconButtonHostView> for CardIconButtonHostModel {
    fn from(view: &CardIconButtonHostView) -> Self {
        let CardIconButtonHostView {
            icon,
            title,
            aria_label,
            on_click,
        } = view.clone();
        Self {
            icon,
            title,
            aria_label,
            on_click,
        }
    }
}

impl ddd::Layered for CardIconButtonHostModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for CardIconButtonHostModel {
    type View = CardIconButtonHostView;
}
