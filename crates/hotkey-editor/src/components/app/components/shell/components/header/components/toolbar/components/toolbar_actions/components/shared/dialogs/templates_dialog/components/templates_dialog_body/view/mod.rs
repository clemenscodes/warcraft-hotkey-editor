use super::TemplatesDialogBody;
use super::components::template_gallery::components::template_card::TemplateCardView;
use super::model::TemplatesDialogBodyModel;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub struct TemplatesDialogBodyView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::View for TemplatesDialogBodyView {}

impl Render for TemplatesDialogBodyView {
    type Model = TemplatesDialogBodyModel;
    type Output = Element;
    fn render(&self) -> Self::Output {
        let cards = self.cards.clone();
        rsx! {
            TemplatesDialogBody {
                cards,
            }
        }
    }
}
