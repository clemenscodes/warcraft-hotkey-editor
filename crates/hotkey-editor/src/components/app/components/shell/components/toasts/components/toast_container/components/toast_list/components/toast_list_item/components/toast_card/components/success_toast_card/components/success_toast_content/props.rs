use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SuccessToastContentProps {
    pub title: String,
    pub description: Option<String>,
}
