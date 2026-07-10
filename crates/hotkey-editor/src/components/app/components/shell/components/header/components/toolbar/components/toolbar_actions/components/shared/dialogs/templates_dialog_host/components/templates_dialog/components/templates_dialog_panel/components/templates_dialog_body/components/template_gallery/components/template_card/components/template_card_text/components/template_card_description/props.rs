use super::view::TemplateCardDescriptionView;
use dioxus::prelude::*;

/// The description line's only input: the template description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardDescriptionProps {
    #[props(into)]
    pub description: String,
}

impl From<&TemplateCardDescriptionView> for TemplateCardDescriptionProps {
    fn from(view: &TemplateCardDescriptionView) -> Self {
        let TemplateCardDescriptionView { description } = view.clone();
        Self { description }
    }
}

impl ddd::Props for TemplateCardDescriptionProps {
    type View = TemplateCardDescriptionView;
}
