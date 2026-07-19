use super::view::InfoPopoverBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoPopoverBackdropModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&InfoPopoverBackdropView> for InfoPopoverBackdropModel {
    fn from(view: &InfoPopoverBackdropView) -> Self {
        let InfoPopoverBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for InfoPopoverBackdropModel {
    type View = InfoPopoverBackdropView;
}
