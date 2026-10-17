use super::view::UnresolvedTitleView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UnresolvedTitleModel {
    pub text: &'static str,
}

impl From<&UnresolvedTitleView> for UnresolvedTitleModel {
    fn from(view: &UnresolvedTitleView) -> Self {
        let UnresolvedTitleView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for UnresolvedTitleModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for UnresolvedTitleModel {
    type View = UnresolvedTitleView;
}
