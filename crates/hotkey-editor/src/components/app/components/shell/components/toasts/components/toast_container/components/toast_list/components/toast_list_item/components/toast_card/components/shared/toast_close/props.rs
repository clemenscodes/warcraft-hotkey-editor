use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCloseProps {
    pub id: usize,
    pub on_remove: Callback<usize>,
}
