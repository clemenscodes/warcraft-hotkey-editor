use super::view::TileLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TileLabelModel {
    pub text: Option<String>,
}

impl From<&TileLabelView> for TileLabelModel {
    fn from(view: &TileLabelView) -> Self {
        let TileLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for TileLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TileLabelModel {
    type View = TileLabelView;
}
