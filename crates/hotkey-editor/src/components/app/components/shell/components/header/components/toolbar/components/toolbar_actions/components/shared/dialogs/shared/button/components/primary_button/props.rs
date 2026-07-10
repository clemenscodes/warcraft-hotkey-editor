use super::view::PrimaryButtonView;
use dioxus::prelude::*;

/// The action button's click handler and label text.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryButtonProps {
    pub onclick: EventHandler<MouseEvent>,
    #[props(into)]
    pub label: String,
}

impl From<&PrimaryButtonView> for PrimaryButtonProps {
    fn from(view: &PrimaryButtonView) -> Self {
        let PrimaryButtonView { onclick, label } = view.clone();
        Self { onclick, label }
    }
}

impl ddd::Props for PrimaryButtonProps {
    type View = PrimaryButtonView;
}
