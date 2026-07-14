use super::view::TemplateCardView;
use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardModel {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}

pub(super) struct TemplateCardPresentation {
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&TemplateCardModel> for TemplateCardPresentation {
    fn from(props: &TemplateCardModel) -> Self {
        let on_apply = props.on_apply;
        let onclick = EventHandler::new(move |_event: MouseEvent| on_apply.call(()));
        Self { onclick }
    }
}

impl From<&TemplateCardView> for TemplateCardModel {
    fn from(view: &TemplateCardView) -> Self {
        let TemplateCardView {
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

impl ddd::Model for TemplateCardModel {
    type View = TemplateCardView;
}
