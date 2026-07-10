use super::view::TemplateCardTextView;
use dioxus::prelude::*;

/// The text block's inputs: the card's name and description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardTextProps {
    pub name: String,
    pub description: String,
}

impl From<&TemplateCardTextView> for TemplateCardTextProps {
    fn from(view: &TemplateCardTextView) -> Self {
        let TemplateCardTextView { name, description } = view.clone();
        Self { name, description }
    }
}

impl ddd::Props for TemplateCardTextProps {
    type View = TemplateCardTextView;
}
