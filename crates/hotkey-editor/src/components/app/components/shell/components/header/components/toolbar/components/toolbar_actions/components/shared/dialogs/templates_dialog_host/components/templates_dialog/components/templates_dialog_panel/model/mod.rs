use super::view::TemplatesDialogPanelView;
use super::components::templates_dialog_body::components::template_gallery::components::template_card::TemplateCardView;
use dioxus::prelude::*;

/// The templates dialog's bordered box: the header title and its close handler above the
/// scrolling body's card views, wrapped in the library `DialogContent` (which carries no
/// project class — this panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct TemplatesDialogPanelModel {
    #[props(into)]
    pub title: String,
    pub on_close: EventHandler<()>,
    pub cards: Vec<TemplateCardView>,
}

impl From<&TemplatesDialogPanelView> for TemplatesDialogPanelModel {
    fn from(view: &TemplatesDialogPanelView) -> Self {
        let TemplatesDialogPanelView {
            title,
            on_close,
            cards,
        } = view.clone();
        Self {
            title,
            on_close,
            cards,
        }
    }
}

impl ddd::Model for TemplatesDialogPanelModel {
    type View = TemplatesDialogPanelView;
}
