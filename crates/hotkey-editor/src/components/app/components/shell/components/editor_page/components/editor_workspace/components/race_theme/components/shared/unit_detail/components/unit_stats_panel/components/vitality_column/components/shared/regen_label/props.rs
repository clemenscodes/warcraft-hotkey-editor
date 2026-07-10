use dioxus::prelude::*;

/// A regeneration label leaf's input: the row name it presents. Both regeneration rows
/// share this dimmer gold and thread only their name.
#[derive(Props, Clone, PartialEq)]
pub struct RegenLabelProps {
    #[props(into)]
    pub text: String,
}
