use super::view::RegenerationHeadingView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct RegenerationHeadingModel {
    pub text: &'static str,
}

impl From<&RegenerationHeadingView> for RegenerationHeadingModel {
    fn from(view: &RegenerationHeadingView) -> Self {
        let RegenerationHeadingView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for RegenerationHeadingModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RegenerationHeadingModel {
    type View = RegenerationHeadingView;
}
