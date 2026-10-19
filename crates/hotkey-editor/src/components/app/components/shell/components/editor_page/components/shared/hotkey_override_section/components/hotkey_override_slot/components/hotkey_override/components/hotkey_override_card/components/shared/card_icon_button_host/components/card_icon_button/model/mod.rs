use super::view::CardIconButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardIconButtonModel {
    #[props(into)]
    pub title: String,
    pub icon: &'static str,
    pub aria_label: &'static str,
    pub on_click: EventHandler<()>,
}

impl From<&CardIconButtonView> for CardIconButtonModel {
    fn from(view: &CardIconButtonView) -> Self {
        let CardIconButtonView {
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

impl ddd::Layered for CardIconButtonModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for CardIconButtonModel {
    type View = CardIconButtonView;
}
