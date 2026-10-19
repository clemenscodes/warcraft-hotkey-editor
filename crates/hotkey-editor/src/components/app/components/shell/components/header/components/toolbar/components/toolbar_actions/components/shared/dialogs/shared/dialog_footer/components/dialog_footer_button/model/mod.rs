use super::view::DialogFooterButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterButtonModel {
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

impl From<&DialogFooterButtonView> for DialogFooterButtonModel {
    fn from(view: &DialogFooterButtonView) -> Self {
        let DialogFooterButtonView { onclick, label } = view.clone();
        Self { onclick, label }
    }
}

impl ddd::Layered for DialogFooterButtonModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for DialogFooterButtonModel {
    type View = DialogFooterButtonView;
}
