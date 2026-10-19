#[derive(Clone, PartialEq)]
pub struct TemplateCardTextView {
    pub name: String,
    pub description: String,
}

impl ddd::Layered for TemplateCardTextView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplateCardTextView {}
