use super::super::shared::template_card::TemplateCardView;
use super::view::TemplateGalleryView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TemplateGalleryModel {
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplateGalleryView> for TemplateGalleryModel {
    fn from(view: &TemplateGalleryView) -> Self {
        let TemplateGalleryView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Layered for TemplateGalleryModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TemplateGalleryModel {
    type View = TemplateGalleryView;
}
