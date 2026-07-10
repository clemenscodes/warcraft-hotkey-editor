use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct WarningToastContentProps {
    pub title: String,
    pub description: Option<String>,
}
