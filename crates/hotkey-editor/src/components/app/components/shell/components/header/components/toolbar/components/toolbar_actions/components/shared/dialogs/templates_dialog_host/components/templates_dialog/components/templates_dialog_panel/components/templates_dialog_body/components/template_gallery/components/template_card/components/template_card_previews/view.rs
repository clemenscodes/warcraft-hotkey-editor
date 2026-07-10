use warcraft_keybinds::ResolvedTemplate;

/// The published `View` contract mirroring [`TemplateCardPreviewsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplateCardPreviewsView {
    pub resolved: ResolvedTemplate,
}

impl ddd::View for TemplateCardPreviewsView {}
