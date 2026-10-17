use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::templates_dialog::components::templates_dialog_body::components::shared::template_card::TemplateCardView;

#[derive(Clone, PartialEq)]
pub struct TemplatePagerView {
    pub cards: Vec<TemplateCardView>,
}

impl ddd::Layered for TemplatePagerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for TemplatePagerView {}
