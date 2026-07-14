use super::components::template_gallery::components::template_card::TemplateCardView;
use super::view::TemplatesDialogBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogBodyModel {
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplatesDialogBodyView> for TemplatesDialogBodyModel {
    fn from(view: &TemplatesDialogBodyView) -> Self {
        let TemplatesDialogBodyView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Model for TemplatesDialogBodyModel {
    type View = TemplatesDialogBodyView;
}
