use super::view::ResolveSectionTabLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolveSectionTabLabelModel {
    #[props(into)]
    pub text: String,
}

impl From<&ResolveSectionTabLabelView> for ResolveSectionTabLabelModel {
    fn from(view: &ResolveSectionTabLabelView) -> Self {
        let ResolveSectionTabLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for ResolveSectionTabLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ResolveSectionTabLabelModel {
    type View = ResolveSectionTabLabelView;
}
