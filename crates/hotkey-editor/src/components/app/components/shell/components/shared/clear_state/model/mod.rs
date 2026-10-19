use super::view::ClearStateView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ClearStateModel {
    pub text: &'static str,
}

impl From<&ClearStateView> for ClearStateModel {
    fn from(view: &ClearStateView) -> Self {
        let ClearStateView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for ClearStateModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ClearStateModel {
    type View = ClearStateView;
}
