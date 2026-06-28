use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CommandGridHeadingProps {
    pub heading: &'static str,
}
