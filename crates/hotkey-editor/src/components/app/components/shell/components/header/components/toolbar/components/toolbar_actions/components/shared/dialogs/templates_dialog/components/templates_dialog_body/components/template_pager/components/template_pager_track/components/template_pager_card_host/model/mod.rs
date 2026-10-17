use super::view::TemplatePagerCardHostView;
use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

#[derive(Props, Clone, PartialEq)]
pub struct TemplatePagerCardHostModel {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}

impl From<&TemplatePagerCardHostView> for TemplatePagerCardHostModel {
    fn from(view: &TemplatePagerCardHostView) -> Self {
        let TemplatePagerCardHostView {
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

impl ddd::Layered for TemplatePagerCardHostModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TemplatePagerCardHostModel {
    type View = TemplatePagerCardHostView;
}
