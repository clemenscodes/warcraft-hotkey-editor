use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

#[derive(Clone, PartialEq)]
pub struct TemplateCardView {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}

impl ddd::View for TemplateCardView {}
