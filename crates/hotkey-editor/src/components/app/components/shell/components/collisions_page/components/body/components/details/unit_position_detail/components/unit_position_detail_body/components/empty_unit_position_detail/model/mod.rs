use super::view::EmptyUnitPositionDetailView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EmptyUnitPositionDetailModel {
    #[props(into)]
    pub prompt: String,
}

impl From<&EmptyUnitPositionDetailView> for EmptyUnitPositionDetailModel {
    fn from(view: &EmptyUnitPositionDetailView) -> Self {
        let EmptyUnitPositionDetailView { prompt } = view.clone();
        Self { prompt }
    }
}

impl ddd::Model for EmptyUnitPositionDetailModel {
    type View = EmptyUnitPositionDetailView;
}
