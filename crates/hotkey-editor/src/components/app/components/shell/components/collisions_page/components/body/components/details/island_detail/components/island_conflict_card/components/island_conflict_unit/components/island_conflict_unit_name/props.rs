use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictUnitNameProps {
    #[props(into)]
    pub text: String,
}
