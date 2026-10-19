use super::view::GridHeadingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct GridHeadingModel {
    pub heading: &'static str,
}

impl From<&GridHeadingView> for GridHeadingModel {
    fn from(view: &GridHeadingView) -> Self {
        let GridHeadingView { heading } = view.clone();
        Self { heading }
    }
}

impl ddd::Layered for GridHeadingModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for GridHeadingModel {
    type View = GridHeadingView;
}
