use super::view::RegenLabelView;
use dioxus::prelude::*;

/// A regeneration label leaf's input: the row name it presents. Both regeneration rows
/// share this dimmer gold and thread only their name.
#[derive(Props, Clone, PartialEq)]
pub struct RegenLabelProps {
    #[props(into)]
    pub text: String,
}

impl From<&RegenLabelView> for RegenLabelProps {
    fn from(view: &RegenLabelView) -> Self {
        let RegenLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for RegenLabelProps {
    type View = RegenLabelView;
}
