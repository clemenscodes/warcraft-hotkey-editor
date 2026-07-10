use super::view::UnitDescriptionView;
use dioxus::prelude::*;

/// The unit's flavor text (its ubertip).
#[derive(Props, Clone, PartialEq)]
pub struct UnitDescriptionProps {
    #[props(into)]
    pub text: String,
}

impl From<&UnitDescriptionView> for UnitDescriptionProps {
    fn from(view: &UnitDescriptionView) -> Self {
        let UnitDescriptionView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for UnitDescriptionProps {
    type View = UnitDescriptionView;
}
