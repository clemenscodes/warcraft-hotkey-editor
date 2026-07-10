use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ErrorToastContentProps {
    pub title: String,
    pub description: Option<String>,
}
