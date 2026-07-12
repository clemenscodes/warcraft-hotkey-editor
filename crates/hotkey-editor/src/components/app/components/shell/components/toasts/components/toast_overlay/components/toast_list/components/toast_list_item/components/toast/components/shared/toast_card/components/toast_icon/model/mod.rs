use super::view::ToastIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToastIconModel {
    pub icon: &'static str,
}

impl From<&ToastIconView> for ToastIconModel {
    fn from(view: &ToastIconView) -> Self {
        let ToastIconView { icon } = view.clone();
        Self { icon }
    }
}

impl ddd::Model for ToastIconModel {
    type View = ToastIconView;
}
