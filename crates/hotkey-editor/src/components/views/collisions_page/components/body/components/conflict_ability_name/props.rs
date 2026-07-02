use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityNameProps {
    #[props(into)]
    pub text: String,
}
