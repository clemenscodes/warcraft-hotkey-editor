use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

#[derive(Clone, PartialEq)]
pub struct TemplateCardPreviewsView {
    pub resolved: ResolvedTemplate,
    pub onscroll: EventHandler<ScrollEvent>,
}

impl ddd::Layered for TemplateCardPreviewsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplateCardPreviewsView {}
