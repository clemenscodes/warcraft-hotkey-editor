use super::view::TemplateCardView;
use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardModel {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}

impl From<&TemplateCardView> for TemplateCardModel {
    fn from(view: &TemplateCardView) -> Self {
        let TemplateCardView {
            name,
            description,
            resolved,
            on_apply,
        } = view.clone();
        Self {
            name,
            description,
            resolved,
            on_apply,
        }
    }
}

impl ddd::Layered for TemplateCardModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TemplateCardModel {
    type View = TemplateCardView;
}
