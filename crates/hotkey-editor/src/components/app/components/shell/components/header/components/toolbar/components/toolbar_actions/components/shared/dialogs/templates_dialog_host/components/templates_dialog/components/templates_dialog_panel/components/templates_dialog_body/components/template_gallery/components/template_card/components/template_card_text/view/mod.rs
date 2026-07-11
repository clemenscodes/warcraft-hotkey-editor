/// The published `View` contract mirroring [`TemplateCardTextModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplateCardTextView {
    pub name: String,
    pub description: String,
}

impl ddd::View for TemplateCardTextView {}
