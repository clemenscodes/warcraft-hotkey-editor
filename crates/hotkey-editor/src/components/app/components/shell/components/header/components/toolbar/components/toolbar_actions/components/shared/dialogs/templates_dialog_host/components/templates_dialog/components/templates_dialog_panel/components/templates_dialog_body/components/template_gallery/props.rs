use super::components::template_card::TemplateCardView;
use dioxus::prelude::*;

/// The gallery's input: the resolved template card views to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateGalleryProps {
    pub cards: Vec<TemplateCardView>,
}
