use super::components::template_gallery::components::template_card::TemplateCardView;
use dioxus::prelude::*;

/// The templates dialog's scroll region input: the resolved template card views its
/// gallery lays out.
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogBodyProps {
    pub cards: Vec<TemplateCardView>,
}
