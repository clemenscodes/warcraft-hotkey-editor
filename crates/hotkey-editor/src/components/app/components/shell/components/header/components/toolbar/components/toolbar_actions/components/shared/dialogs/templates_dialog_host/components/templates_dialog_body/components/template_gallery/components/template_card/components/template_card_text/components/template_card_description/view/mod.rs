/// The published `View` contract mirroring [`TemplateCardDescriptionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplateCardDescriptionView {
    pub description: String,
}

impl ddd::View for TemplateCardDescriptionView {}
