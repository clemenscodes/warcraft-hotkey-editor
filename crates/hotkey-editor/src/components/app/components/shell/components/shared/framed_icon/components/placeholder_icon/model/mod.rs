use super::view::PlaceholderIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PlaceholderIconModel {
    pub source: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&PlaceholderIconView> for PlaceholderIconModel {
    fn from(view: &PlaceholderIconView) -> Self {
        let PlaceholderIconView { source, alt } = view.clone();
        Self { source, alt }
    }
}

impl ddd::Layered for PlaceholderIconModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PlaceholderIconModel {
    type View = PlaceholderIconView;
}
