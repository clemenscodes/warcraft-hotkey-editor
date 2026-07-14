use warcraft_keybinds::ResolvedTemplate;

#[derive(Clone, PartialEq)]
pub struct TemplateCardPreviewsView {
    pub resolved: ResolvedTemplate,
}

impl ddd::View for TemplateCardPreviewsView {}
