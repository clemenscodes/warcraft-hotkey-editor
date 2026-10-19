use super::view::EmptyDetailView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyDetailModel {
    #[props(into)]
    pub prompt: String,
}

impl From<&EmptyDetailView> for EmptyDetailModel {
    fn from(view: &EmptyDetailView) -> Self {
        let EmptyDetailView { prompt } = view.clone();
        Self { prompt }
    }
}

impl ddd::Layered for EmptyDetailModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for EmptyDetailModel {
    type View = EmptyDetailView;
}
