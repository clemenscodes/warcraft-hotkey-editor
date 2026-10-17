use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

#[derive(Clone, PartialEq)]
pub struct TemplateCardView {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}

impl ddd::Layered for TemplateCardView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplateCardView {}
