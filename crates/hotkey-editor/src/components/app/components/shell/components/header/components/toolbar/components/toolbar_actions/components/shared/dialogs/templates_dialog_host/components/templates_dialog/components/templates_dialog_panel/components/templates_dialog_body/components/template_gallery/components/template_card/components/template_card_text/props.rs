use dioxus::prelude::*;

/// The text block's inputs: the card's name and description.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardTextProps {
    pub name: String,
    pub description: String,
}
