use super::view::TemplatePagerTrackView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::components::templates_dialog_body::components::shared::template_card::TemplateCardView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TemplatePagerTrackModel {
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplatePagerTrackView> for TemplatePagerTrackModel {
    fn from(view: &TemplatePagerTrackView) -> Self {
        let TemplatePagerTrackView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Layered for TemplatePagerTrackModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for TemplatePagerTrackModel {
    type View = TemplatePagerTrackView;
}
