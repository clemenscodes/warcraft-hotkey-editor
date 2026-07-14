#[derive(Clone, PartialEq)]
pub struct TemplateCardTextView {
    pub name: String,
    pub description: String,
}

impl ddd::View for TemplateCardTextView {}
