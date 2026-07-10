use super::components::template_gallery::components::template_card::TemplateCardView;
use super::view::TemplatesDialogBodyView;
use dioxus::prelude::*;

/// The templates dialog's scroll region input: the resolved template card views its
/// gallery lays out.
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogBodyProps {
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplatesDialogBodyView> for TemplatesDialogBodyProps {
    fn from(view: &TemplatesDialogBodyView) -> Self {
        let TemplatesDialogBodyView { cards } = view.clone();
        Self { cards }
    }
}

impl ddd::Props for TemplatesDialogBodyProps {
    type View = TemplatesDialogBodyView;
}
