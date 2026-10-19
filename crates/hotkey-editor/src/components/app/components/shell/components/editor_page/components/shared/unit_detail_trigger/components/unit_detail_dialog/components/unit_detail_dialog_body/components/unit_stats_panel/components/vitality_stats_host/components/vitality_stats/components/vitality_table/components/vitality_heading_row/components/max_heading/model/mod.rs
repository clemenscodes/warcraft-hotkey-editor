use super::view::MaxHeadingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct MaxHeadingModel {
    pub text: &'static str,
}

impl From<&MaxHeadingView> for MaxHeadingModel {
    fn from(view: &MaxHeadingView) -> Self {
        let MaxHeadingView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for MaxHeadingModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for MaxHeadingModel {
    type View = MaxHeadingView;
}
