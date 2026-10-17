#[derive(Clone, PartialEq)]
pub struct TemplateCardDescriptionView {
    pub description: String,
}

impl ddd::Layered for TemplateCardDescriptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplateCardDescriptionView {}
