use super::view::SecondaryButtonView;
use dioxus::prelude::*;

/// The action button's click handler and label text.
#[derive(Props, Clone, PartialEq)]
pub struct SecondaryButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

impl From<&SecondaryButtonView> for SecondaryButtonProps {
    fn from(view: &SecondaryButtonView) -> Self {
        let SecondaryButtonView { onclick, label } = view.clone();
        Self { onclick, label }
    }
}

impl ddd::Props for SecondaryButtonProps {
    type View = SecondaryButtonView;
}
