use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictUnitNameProps {
    #[props(into)]
    pub text: String,
}
