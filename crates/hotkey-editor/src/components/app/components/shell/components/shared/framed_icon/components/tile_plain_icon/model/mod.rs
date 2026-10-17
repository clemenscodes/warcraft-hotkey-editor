use super::view::TilePlainIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TilePlainIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&TilePlainIconView> for TilePlainIconModel {
    fn from(view: &TilePlainIconView) -> Self {
        let TilePlainIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Layered for TilePlainIconModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TilePlainIconModel {
    type View = TilePlainIconView;
}
