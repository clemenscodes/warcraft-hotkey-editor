use dioxus::prelude::*;

use super::components::template_card::TemplateCardProps;

/// The gallery's input: the resolved template cards to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateGalleryProps {
    pub cards: Vec<TemplateCardProps>,
}
