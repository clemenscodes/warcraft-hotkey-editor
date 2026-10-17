use super::view::TemplateCardNameView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardNameModel {
    #[props(into)]
    pub name: String,
}

impl From<&TemplateCardNameView> for TemplateCardNameModel {
    fn from(view: &TemplateCardNameView) -> Self {
        let TemplateCardNameView { name } = view.clone();
        Self { name }
    }
}

impl ddd::Layered for TemplateCardNameModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TemplateCardNameModel {
    type View = TemplateCardNameView;
}
