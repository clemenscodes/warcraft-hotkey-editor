/// The published `View` contract mirroring [`TemplateCardNameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplateCardNameView {
    pub name: String,
}

impl ddd::View for TemplateCardNameView {}
