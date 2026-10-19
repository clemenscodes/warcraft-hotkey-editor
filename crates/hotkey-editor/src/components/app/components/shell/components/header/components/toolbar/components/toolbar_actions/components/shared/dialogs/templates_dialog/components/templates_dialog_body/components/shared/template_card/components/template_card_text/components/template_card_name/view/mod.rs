#[derive(Clone, PartialEq)]
pub struct TemplateCardNameView {
    pub name: String,
}

impl ddd::Layered for TemplateCardNameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplateCardNameView {}
