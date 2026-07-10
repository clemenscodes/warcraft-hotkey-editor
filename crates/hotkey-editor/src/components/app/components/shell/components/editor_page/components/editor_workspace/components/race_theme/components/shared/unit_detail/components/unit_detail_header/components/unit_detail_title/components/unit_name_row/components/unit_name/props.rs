use super::view::UnitNameView;
use dioxus::prelude::*;

/// The unit's display name.
#[derive(Props, Clone, PartialEq)]
pub struct UnitNameProps {
    pub text: &'static str,
}

impl From<&UnitNameView> for UnitNameProps {
    fn from(view: &UnitNameView) -> Self {
        let UnitNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for UnitNameProps {
    type View = UnitNameView;
}
