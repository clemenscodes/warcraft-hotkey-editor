use crate::components::app::components::shell::components::toasts::ToastRecord;
use dioxus::prelude::*;
use dioxus_kit::toast::ToastProviderModel;

#[derive(Props, Clone, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<ToastRecord>,
    pub on_remove: Callback<usize>,
}

impl From<&ToastProviderModel> for ToastContainerProps {
    fn from(model: &ToastProviderModel) -> Self {
        let toasts = model.records();
        let on_remove = model.on_remove();
        Self { toasts, on_remove }
    }
}
