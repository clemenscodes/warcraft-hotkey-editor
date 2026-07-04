use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct FightIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}
