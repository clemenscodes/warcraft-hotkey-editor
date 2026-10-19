use super::view::InfoPopoverView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoPopoverModel {
    pub text: &'static str,
}

impl From<&InfoPopoverView> for InfoPopoverModel {
    fn from(view: &InfoPopoverView) -> Self {
        let InfoPopoverView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for InfoPopoverModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for InfoPopoverModel {
    type View = InfoPopoverView;
}
