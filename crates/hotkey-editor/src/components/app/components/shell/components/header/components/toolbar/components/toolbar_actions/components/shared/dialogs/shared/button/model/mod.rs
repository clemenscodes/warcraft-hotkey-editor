use super::state::ButtonVariant;
use super::view::ButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonModel {
    pub variant: ButtonVariant,
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

impl From<&ButtonView> for ButtonModel {
    fn from(view: &ButtonView) -> Self {
        let ButtonView {
            variant,
            onclick,
            label,
        } = view.clone();
        Self {
            variant,
            onclick,
            label,
        }
    }
}

impl ddd::Model for ButtonModel {
    type View = ButtonView;
}
