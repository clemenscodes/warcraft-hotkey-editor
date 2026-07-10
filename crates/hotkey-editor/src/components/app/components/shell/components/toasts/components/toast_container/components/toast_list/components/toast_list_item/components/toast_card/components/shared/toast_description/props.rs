use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastDescriptionProps {
    pub description: Option<String>,
}
