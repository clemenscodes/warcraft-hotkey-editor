use super::view::ToastCardView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastCardModel {
    pub icon: &'static str,
    pub title: String,
    pub description: Option<String>,
    pub id: usize,
    pub on_remove: Callback<usize>,
}

impl From<&ToastCardView> for ToastCardModel {
    fn from(view: &ToastCardView) -> Self {
        let ToastCardView {
            icon,
            title,
            description,
            id,
            on_remove,
        } = view.clone();
        Self {
            icon,
            title,
            description,
            id,
            on_remove,
        }
    }
}

impl ddd::Model for ToastCardModel {
    type View = ToastCardView;
}
