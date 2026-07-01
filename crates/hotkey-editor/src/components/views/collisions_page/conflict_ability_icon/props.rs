use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ConflictAbilityIconProps {
    #[props(into)]
    pub src: String,
    #[props(into)]
    pub alt: String,
}
