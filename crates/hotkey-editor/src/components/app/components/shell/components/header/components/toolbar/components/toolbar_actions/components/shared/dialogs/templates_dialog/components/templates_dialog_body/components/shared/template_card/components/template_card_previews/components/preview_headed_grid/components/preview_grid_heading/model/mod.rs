use super::view::PreviewGridHeadingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewGridHeadingModel {
    pub heading: &'static str,
}

impl From<&PreviewGridHeadingView> for PreviewGridHeadingModel {
    fn from(view: &PreviewGridHeadingView) -> Self {
        let PreviewGridHeadingView { heading } = view.clone();
        Self { heading }
    }
}

impl ddd::Layered for PreviewGridHeadingModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PreviewGridHeadingModel {
    type View = PreviewGridHeadingView;
}
