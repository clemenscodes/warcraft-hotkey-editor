use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InfoToastContentProps {
    pub title: String,
    pub description: Option<String>,
}
