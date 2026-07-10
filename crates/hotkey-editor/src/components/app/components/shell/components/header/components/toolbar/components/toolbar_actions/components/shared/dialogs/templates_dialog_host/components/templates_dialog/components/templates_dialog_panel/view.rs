use super::components::templates_dialog_body::components::template_gallery::components::template_card::TemplateCardView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`TemplatesDialogPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TemplatesDialogPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub cards: Vec<TemplateCardView>,
}

impl ddd::View for TemplatesDialogPanelView {}
