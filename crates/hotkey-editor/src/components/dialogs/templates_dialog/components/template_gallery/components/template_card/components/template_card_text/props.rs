use super::super::super::TemplateCardProps;
use dioxus::prelude::*;

/// The text block's inputs: the card's name and description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardTextProps {
    pub name: String,
    pub description: String,
}

impl From<&TemplateCardProps> for TemplateCardTextProps {
    fn from(props: &TemplateCardProps) -> Self {
        let name = props.name.clone();
        let description = props.description.clone();
        Self { name, description }
    }
}
