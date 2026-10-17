use super::view::TemplatePagerView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::components::templates_dialog_body::components::shared::template_card::TemplateCardView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TemplatePagerModel {
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplatePagerView> for TemplatePagerModel {
    fn from(view: &TemplatePagerView) -> Self {
        let TemplatePagerView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Layered for TemplatePagerModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TemplatePagerModel {
    type View = TemplatePagerView;
}
