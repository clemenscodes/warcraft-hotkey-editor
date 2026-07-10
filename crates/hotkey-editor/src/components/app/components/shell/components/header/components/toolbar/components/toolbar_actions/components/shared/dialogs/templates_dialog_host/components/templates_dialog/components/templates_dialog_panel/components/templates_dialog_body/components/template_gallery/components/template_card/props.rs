use super::view::TemplateCardView;
use dioxus::prelude::*;
use warcraft_keybinds::ResolvedTemplate;

/// One template card's inputs: its name and description, the resolved layout it
/// previews, and the handler that applies it.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardProps {
    pub name: String,
    pub description: String,
    pub resolved: ResolvedTemplate,
    pub on_apply: EventHandler<()>,
}

/// The card's click handler, adapted from `on_apply` so the body only places it.
pub(super) struct TemplateCardPresentation {
    pub(super) onclick: EventHandler<MouseEvent>,
}

impl From<&TemplateCardProps> for TemplateCardPresentation {
    fn from(props: &TemplateCardProps) -> Self {
        let on_apply = props.on_apply;
        let onclick = EventHandler::new(move |_event: MouseEvent| on_apply.call(()));
        Self { onclick }
    }
}

impl From<&TemplateCardView> for TemplateCardProps {
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

impl ddd::Props for TemplateCardProps {
    type View = TemplateCardView;
}
