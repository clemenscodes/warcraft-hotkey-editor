use super::view::DialogFooterView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&DialogFooterView> for DialogFooterModel {
    fn from(view: &DialogFooterView) -> Self {
        let DialogFooterView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Layered for DialogFooterModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for DialogFooterModel {
    type View = DialogFooterView;
}
