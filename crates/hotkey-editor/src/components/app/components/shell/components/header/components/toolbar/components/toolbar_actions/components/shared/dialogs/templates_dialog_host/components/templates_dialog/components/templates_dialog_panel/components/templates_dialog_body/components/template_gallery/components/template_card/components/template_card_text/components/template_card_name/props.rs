use super::view::TemplateCardNameView;
use dioxus::prelude::*;

/// The name heading's only input: the template name.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardNameProps {
    #[props(into)]
    pub name: String,
}

impl From<&TemplateCardNameView> for TemplateCardNameProps {
    fn from(view: &TemplateCardNameView) -> Self {
        let TemplateCardNameView { name } = view.clone();
        Self { name }
    }
}

impl ddd::Props for TemplateCardNameProps {
    type View = TemplateCardNameView;
}
