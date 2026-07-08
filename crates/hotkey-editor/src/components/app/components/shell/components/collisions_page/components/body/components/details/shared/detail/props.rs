use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DetailProps {
    #[props(default)]
    pub is_empty: bool,
    pub children: Element,
}
