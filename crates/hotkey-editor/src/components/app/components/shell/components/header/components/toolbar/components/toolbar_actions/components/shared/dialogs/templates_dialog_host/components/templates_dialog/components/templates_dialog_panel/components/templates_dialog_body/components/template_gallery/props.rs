use super::components::template_card::TemplateCardView;
use super::view::TemplateGalleryView;
use dioxus::prelude::*;

/// The gallery's input: the resolved template card views to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateGalleryProps {
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplateGalleryView> for TemplateGalleryProps {
    fn from(view: &TemplateGalleryView) -> Self {
        let TemplateGalleryView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Props for TemplateGalleryProps {
    type View = TemplateGalleryView;
}
