use super::components::template_card::TemplateCardView;

#[derive(Clone, PartialEq)]
pub struct TemplateGalleryView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::Layered for TemplateGalleryView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplateGalleryView {}
