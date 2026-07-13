use super::components::template_card::TemplateCardView;

/// The published `View` contract mirroring [`TemplateGalleryModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplateGalleryView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::View for TemplateGalleryView {}
