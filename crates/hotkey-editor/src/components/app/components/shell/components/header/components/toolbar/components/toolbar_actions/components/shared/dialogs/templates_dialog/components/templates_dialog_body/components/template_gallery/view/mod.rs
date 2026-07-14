use super::components::template_card::TemplateCardView;

#[derive(Clone, PartialEq)]
pub struct TemplateGalleryView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::View for TemplateGalleryView {}
