use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
