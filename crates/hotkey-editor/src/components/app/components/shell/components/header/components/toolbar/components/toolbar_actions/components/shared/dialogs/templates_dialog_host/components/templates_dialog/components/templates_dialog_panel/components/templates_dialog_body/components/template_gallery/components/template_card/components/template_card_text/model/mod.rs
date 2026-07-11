use super::view::TemplateCardTextView;
use dioxus::prelude::*;

/// The text block's inputs: the card's name and description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardTextModel {
    pub name: String,
    pub description: String,
}

impl From<&TemplateCardTextView> for TemplateCardTextModel {
    fn from(view: &TemplateCardTextView) -> Self {
        let TemplateCardTextView { name, description } = view.clone();
        Self { name, description }
    }
}

impl ddd::Model for TemplateCardTextModel {
    type View = TemplateCardTextView;
}
