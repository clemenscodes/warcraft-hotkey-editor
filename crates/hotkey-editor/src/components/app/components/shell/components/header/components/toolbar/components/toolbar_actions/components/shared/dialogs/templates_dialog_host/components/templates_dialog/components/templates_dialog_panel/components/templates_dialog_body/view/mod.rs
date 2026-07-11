use super::components::template_gallery::components::template_card::TemplateCardView;

/// The published `View` contract mirroring [`TemplatesDialogBodyModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplatesDialogBodyView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::View for TemplatesDialogBodyView {}
