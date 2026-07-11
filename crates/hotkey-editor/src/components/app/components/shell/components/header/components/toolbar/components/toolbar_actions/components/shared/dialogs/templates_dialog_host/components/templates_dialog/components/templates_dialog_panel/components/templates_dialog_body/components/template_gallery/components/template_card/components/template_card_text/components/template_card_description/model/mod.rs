use super::view::TemplateCardDescriptionView;
use dioxus::prelude::*;

/// The description line's only input: the template description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardDescriptionModel {
    #[props(into)]
    pub description: String,
}

impl From<&TemplateCardDescriptionView> for TemplateCardDescriptionModel {
    fn from(view: &TemplateCardDescriptionView) -> Self {
        let TemplateCardDescriptionView { description } = view.clone();
        Self { description }
    }
}

impl ddd::Model for TemplateCardDescriptionModel {
    type View = TemplateCardDescriptionView;
}
