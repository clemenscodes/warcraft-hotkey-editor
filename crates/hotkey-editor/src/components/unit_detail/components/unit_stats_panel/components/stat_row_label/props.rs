use dioxus::prelude::*;

/// A stat row's label (e.g. "Hit Points").
#[derive(Props, Clone, PartialEq)]
pub struct StatRowLabelProps {
    #[props(into)]
    pub text: String,
}
