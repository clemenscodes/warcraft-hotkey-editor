use super::view::RegenQualifierView;
use dioxus::prelude::*;

/// The regen qualifier ("at night" / "on blight") shown before the regen gain.
#[derive(Props, Clone, PartialEq)]
pub struct RegenQualifierProps {
    #[props(default)]
    pub text: Option<&'static str>,
}

impl From<&RegenQualifierView> for RegenQualifierProps {
    fn from(view: &RegenQualifierView) -> Self {
        let RegenQualifierView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for RegenQualifierProps {
    type View = RegenQualifierView;
}
