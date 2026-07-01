use super::components::template_card::TemplateCardProps;
use dioxus::prelude::*;

/// The gallery's input: the resolved template cards to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateGalleryProps {
    pub cards: Vec<TemplateCardProps>,
}
